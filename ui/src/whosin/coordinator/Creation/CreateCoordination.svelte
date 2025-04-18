<script lang="ts">
  import { createEventDispatcher, getContext } from 'svelte';
  import { type AppClient, type Record } from '@holochain/client';
  import { clientContext } from '../../../contexts';
  import type { Coordination, Coordrole } from '../types';
  import '@material/mwc-button';
  import '@material/mwc-snackbar';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-textfield';
  import '@material/mwc-textarea';
  import { view, viewHash, navigate } from '../../../store.js';
  import AttachmentsDialog from '../../../AttachmentsDialog.svelte';
  import { isWeContext } from '@lightningrodlabs/we-applet';
  import { countViewed, addToViewed } from '../../../store.js';
  import type { WALUrl } from '../../../util';
  import { getMyDna } from '../../../util';
  import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
  import SvgIcon from '../../../SvgIcon.svelte';
  import type { WAL } from '@lightningrodlabs/we-applet';
  import { onMount } from 'svelte';
  import { weClientStored } from '../../../store.js';
  import { getTime, secondsToDateInput } from './helper';
  export let agreementType: string = "event";
  import CreateRole from './CreateRole.svelte';
  import { cloneDeep } from 'lodash';

  export let fromCalendar = false;

  let client: AppClient = (getContext(clientContext) as any).getClient();
  
  let weClient: any;
  weClientStored.subscribe(value => {
    weClient = value;
  });

  const dispatch = createEventDispatcher();
  
  let showDescription = false;
  let repeat = '';
  let editingType = 'only me';
  let postToBulletin = true;
  let attachmentsDialog : AttachmentsDialog
  let attachments: Array<WALUrl> = [];
  let title: string | undefined;
  let description: string | undefined = '';
  // export let startsDate: number | undefined = agreementType == "event" ? new Date().getTime() * 1000 : undefined;
  export let startsDate: number | undefined = agreementType == "event" ? new Date().getTime() : undefined;
  // $: startsDateString = startsDate ? secondsToDateInput(startsDate) : "";
  let startsDateString: string;
  $: if (startsDate) {
    console.log("startsDate", startsDate)
    startsDateString = secondsToDateInput(startsDate);
  } else {
    startsDateString = "";
  }

  export let endsDate: number | undefined = agreementType == "event" ? new Date().getTime() + 3600 : undefined;
  let endsDateString: string;
  $: if (endsDate) {
    endsDateString = secondsToDateInput(endsDate);
  } else {
    endsDateString = "";
  }

  let signupDeadline: number | undefined;
  let deadlineDateString: string;
  $: if (signupDeadline) {
    deadlineDateString = secondsToDateInput(signupDeadline);
  } else {
    deadlineDateString = "";
  }

  let reminderDate: number | undefined;
  let coordRoles: Coordrole[] = [];
  let roleTitle: string | undefined;
  let roleDescription: string | undefined;
  let minimum: number | undefined;
  let maximum: number | undefined;
  let notifierVisibility = false;
  let titleField;
  const agreementTypeGrammar = {
    "event": "an event",
    "project": "a project",
    "ongoing agreement": "an ongoing agreement"
  }
  
  let errorSnackbar: Snackbar;
  let dnaHash;
  
  $: title, description, startsDate, endsDate, signupDeadline, reminderDate, coordRoles, roleTitle, roleDescription, minimum, maximum, attachments;
  $: isCoordinationValid = title !== undefined && description !== undefined && coordRoles.length > 0 && (agreementType != "event" || (startsDateString != undefined)) && (!endsDateString || startsDateString <= endsDateString) //&& happeningDate !== undefined && signupDeadline !== undefined && reminderDate !== undefined;//
  // $: isCoordRoleValid = roleTitle != undefined && roleDescription != undefined && minimum != undefined && maximum != undefined && minimum <= maximum && minimum >= 0;
  $: areCoordRolesValid = coordRoles.every(role => {
    return role.title != undefined && role.title.length > 0 && role.description != undefined && role.minimum != undefined && (!role.maximum || role.minimum <= role.maximum) && role.minimum >= 0;
  });

  async function createCoordination() {
    const coordinationEntry: Coordination = {
      title: title!,
      description: description!,
      // capitalized agreementType
      coordination_type: agreementType!,
      starts_date: startsDateString ? new Date(startsDateString).valueOf() * 1000 : undefined,
      ends_date: endsDateString ? new Date(endsDateString).valueOf() * 1000 : undefined,
      signup_deadline: deadlineDateString ? new Date(deadlineDateString).valueOf() * 1000 : undefined,
      // reminder_date: reminderDate!,
      coordroles: coordRoles!,
      attachments: attachments
    };
    
    try {
      console.log("coordinationEntry", coordinationEntry)

      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'create_coordination',
        payload: coordinationEntry
      });

      addToViewed(record.signed_action.hashed.hash, client)
  
      dispatch('coordination-created', { 
        coordinationHash: record.signed_action.hashed.hash 
      });
  
      const wal: WAL = { hrl: [dnaHash, record.signed_action.hashed.hash], context: "" }
      console.log(weClient, weClient?.renderInfo)
      try {
        weClient?.renderInfo.view.resolve(wal)
      } catch (e) {
        console.log("Created coordination")
      }

      if (fromCalendar) {
        dispatch('coordination-created', { 
          coordinationHash: record.signed_action.hashed.hash 
        });
      } else {
        navigate("coordination", record.signed_action.hashed.hash);
      }
      
    } catch (e) {
      errorSnackbar.labelText = `Error creating the coordination: ${e}`;
      errorSnackbar.show();
    }
  }

  async function claimNotifier() {
    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'claim_notifier',
        payload: null,
      });

    } catch (e) {
      errorSnackbar.labelText = `Error creating the coordination: ${e}`;
      errorSnackbar.show();
    }
  }

  async function notifierPopup() {
    notifierVisibility = true;
  }
  
  async function addCoordrole() {
    // roleTitle = undefined;
    // roleDescription = undefined;
    // minimum = 1;
    // maximum = undefined;
    coordRoles.push({title: "", description: "", minimum: 1, maximum: undefined});
    coordRoles = coordRoles;
  }

  async function scrollToBottom() {
    if (typeof window !== 'undefined') {
      // await new Promise(res => setTimeout(res, 100));
      window.scrollTo(0, document.body.scrollHeight);
      // await new Promise(res => setTimeout(res, 500));
      // window.scrollTo(0, document.body.scrollHeight);
    }
  }
  
  async function removeRole(role:Coordrole) {
    let index = coordRoles.indexOf(role)
    coordRoles.splice(index, 1)
    coordRoles = coordRoles
  }

  onMount(async () => {
    // startsDate = new Date().getTime() * 1000;
    // startsDateString = secondsToDateInput(startsDate);

    console.log("test date", new Date().getTime() * 1000, startsDate, startsDateString, secondsToDateInput(new Date().getTime() * 1000))
    dnaHash = await getMyDna("whosin", client);
    titleField.focus();
    if (agreementType == "event") {
      // startsDate = new Date().valueOf() * 1000;
      // endsDate = new Date().valueOf() * 1000 + 3600;
      coordRoles.push({title: "Participant", description: "", minimum: 1, maximum: undefined});
      coordRoles = coordRoles;
    } else if (agreementType == "project") {
      coordRoles.push({title: "Participant", description: "", minimum: 1, maximum: undefined});
      coordRoles = coordRoles;
    } else if (agreementType == "agreement") {
      coordRoles.push({title: "Signatory", description: "", minimum: 1, maximum: undefined});
      coordRoles = coordRoles;
    }
  });
  
  </script>
  
  <mwc-snackbar bind:this={errorSnackbar} leading>
  </mwc-snackbar>
  

    <!-- {:else} -->
      {#if !agreementType}
        <h3>
          Coordination type
        </h3>
      <div class="choose-type">
        <div on:click={() => {agreementType = "event"}} style="margin-right: 8px; background: {agreementType == "event" ? "#dfe4e9" : "#fff"}">
          <SvgIcon icon="faCalendar" />
          Event</div>
        <div on:click={() => {agreementType = "project"}} style="background: {agreementType == "project" ? "#dfe4e9" : "#fff"}">
          <SvgIcon icon="faTask" />
          Project</div>
        <div on:click={() => {agreementType = "agreement"}} style="margin-left: 8px; background: {agreementType == "agreement" ? "#dfe4e9" : "#fff"}">
          <SvgIcon icon="faAgreement" />
          Ongoing agreement</div>
      </div>
    {/if}

    {#if agreementType}

      <!-- <h1 style="font-size: 24px; font-weight: 400; text-align: left;">Create {agreementTypeGrammar[agreementType]}</h1> -->
    
      <h3 style="text-transform: capitalize; margin-bottom: 0;">New {agreementType}</h3>

      <p class="notice">Warning: After proposing an {agreementType}, it "belongs" to everyone and cannot be edited or deleted.</p>

      <div style="margin-bottom: 16px; text-align: left;">
        <input class="title-input" placeholder="Title" bind:this={titleField} on:input={e => { title = e.target.value; } } />
      </div>
      
      <div class="optional-fields">
        {#if agreementType == "event"}
          <div class="dates">
            <div class="optional-field">
              <!-- <div class="optional-field"> -->
                <!-- datetime -->
                <span style="text-transform: capitalize">
                  {agreementType}
                </span> starts
                <input type="datetime-local" id="start-date" name="start-date" 
                bind:value={startsDateString}
                required>

                <span style="font-weight: 300; font-size: 14px;">
                  {!endsDate ? "(all-day event)" : ""}
                </span>
                <!-- {JSON.stringify(secondsToDateInput(startsDate))}hi -->
        
                <!-- <vaadin-date-time-picker label="Starts"  on:change={e => { startsDate = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>           -->
              <!-- </div> -->
              <!-- </div>

              <div class="dates"> -->
              {#if !endsDate}
                <div style="margin-bottom: 16px; text-align: left;">
                  <button class="optional-button" on:click={() => endsDate = new Date().valueOf() * 1000}>
                    + event ends
                  </button>
                </div>
              {:else}
                    <!-- <button class="optional-button" on:click={() => endsDate = undefined}>
                      × event ends
                    </button> -->
                    
                  <!-- <div class="optional-field"> -->
                    <span style="text-transform: capitalize">
                    {agreementType}
                    </span> ends
                    <input type="datetime-local" id="end-date" name="end-date"
                    bind:value={endsDateString}
                    on:input={e => { 
                      // const enteredDateTime = cloneDeep(e.target.value);
                      // let newEndsDate = new Date(enteredDateTime).valueOf() * 1000;
                      // console.log("endsDate", enteredDateTime, newEndsDate)
                      // // if (newEndsDate >= startsDate) {
                      // endsDate = newEndsDate;
                      // } else {
                      // e.target.value = null
                      // }
                    } } required>
                  <!-- </div> -->
              {/if}
                  <!-- {JSON.stringify(secondsToDateInput(endsDate))}ho -->
            </div>
          </div>
          {:else if agreementType == "project"}
          <div style="margin-bottom: 16px; text-align: left;">
            Deadline to complete (optional)
            <input type="datetime-local" id="deadline-date" name="deadline-date"
              bind:value={endsDateString}
              on:input={e => { 
                // let newEndsDate = new Date(e.target.value).valueOf() * 1000;
                // if (!startsDate || newEndsDate <= startsDate) {
                //   endsDate = newEndsDate;
                // } else {
                //   e.target.value = null
                // }
              } } 
              required>
          </div>
          {/if}     

          <!-- <div style="margin-bottom: 16px; text-align: left;">
            Reminder
            <input type="datetime-local" id="reminder-date" name="reminder-date" on:input={e => { reminderDate = new Date(e.target.value).valueOf() * 1000;} } required>
          </div> -->

        <div class="dates">
          {#if !signupDeadline}
            <div style="margin-bottom: 16px; text-align: left;">
              <button class="optional-button" on:click={() => signupDeadline = new Date().valueOf() * 1000}>
                + deadline to sign up
              </button>
            </div>
          {:else}
            <div style="margin: 0px;">
              <button class="optional-button" on:click={() => signupDeadline = undefined}>
                × deadline
              </button>
              
              <div class="optional-field">
                Deadline to signup
                <input type="datetime-local" id="signup-deadline" name="signup-deadline" 
                bind:value={deadlineDateString}
                on:input={e => {
                  // signupDeadline = new Date(e.target.value).valueOf() * 1000;
                }} required>
                  <!-- remove? -->
              </div>
            </div>
          {/if}
        </div>

        <!-- <div class="dates">
          {#if !repeat}
            <div style="margin-bottom: 16px; text-align: left;">
              <button class="optional-button" on:click={() => repeat = true}>
                + repeats
              </button>
            </div>
          {:else}
            <div style="margin: 0;">
              <button class="optional-button" on:click={() => repeat = undefined}>
                × repeats
              </button>
              <div class="optional-field">
                Repeat
                <select id="repeat" name="repeat" on:input={e => { repeat = e.target.value;} } required>
                  <option value="daily">Daily</option>
                  <option value="weekly">Weekly</option>
                  <option value="monthly">Monthly</option>
                  <option value="yearly">Yearly</option>
                </select>
              </div>
            </div>
          {/if}
        </div> -->

        {#if !showDescription}
          <div style="margin-bottom: 16px; text-align: left;">
            <button class="optional-button" on:click={() => showDescription = true}>+ description</button>
          </div>
        {:else}
          <div style="margin: 0; text-align: left;">
            <button class="optional-button" on:click={() => showDescription = false}>× description</button>
            <textarea class="description-input" placeholder="Description"  on:input={e => { description = e.target.value;} } />
          </div>
        {/if}

        {#if isWeContext()}
          <div style="display:flex; flex-wrap:wrap; align-items: center; margin-bottom:10px;">
            <AttachmentsDialog bind:this={attachmentsDialog} bind:attachments on:add-attachments={
              (e) => {
                console.log("add-attachments", e.detail)
                attachments = e.detail.attachments
              }
            }></AttachmentsDialog>
          </div>
        {/if}
      </div>


      <div style="display: flex; flex-direction: column; margin-bottom: 1em;">
        <div id="created-roles">
          <h2
            style="margin-top: 0.2em;"
          >Roles for this {agreementType}</h2>
          {#each coordRoles as role, index}
          <div class="role-outer">
            <CreateRole {role} on:update={e => {
                console.log("update role", e.detail)
                let updatedRole = e.detail;
                coordRoles[index] = updatedRole;
                coordRoles = coordRoles;
              }
            }
            />
            <!-- <div>
              <input type="text" value={role.title} on:input={e => role.title = e.target.value} />
              <br>
              <textarea on:input={e => role.description = e.target.value}>{role.description}</textarea>
              <br>
              <div>
              Min: <input type="number" value={role.minimum} on:input={e => role.minimum = +e.target.value} />
              Max: <input type="number" value={role.maximum} on:input={e => role.maximum = +e.target.value} />
              </div>
              <br>
            </div> -->
            <!-- {#if coordRoles.length > 1} -->
             <div style="
              display: flex;
              flex-direction: row;
              width: 100%;
              justify-content: flex-end;
             ">
               <button class="delete" on:click={() => removeRole(role)}>× remove</button>
             </div>
            <!-- {/if} -->
          </div>
          {/each}
          <!-- add role div/button -->
            <div class="add-role-button" on:click={addCoordrole}>
              + Add role
            </div>
        </div>
      </div>

      <!-- <h2
        style="margin-top: 16px;"
      >Add Role</h2>
      <div class="role">
        <input placeholder="Role Title" style="width: 78%" id="role-title-field"
          bind:value={roleTitle}
          />

        <div style="display: flex; margin: 0;">
          <input placeholder="Min" style="width: 60px" type="number" id="minimum-field"
          bind:value={minimum}
          />
          <input placeholder="Max" style="width: 60px" type="number" id="maximum-field"
          bind:value={maximum}
          />
        </div>
      </div>
      <div class="role">
        <input placeholder="Role Description" style="width: 88%" id="role-title-field"
        bind:value={roleDescription}
        />
        
        <button class="add-role"
          disabled={!isCoordRoleValid}
          on:click={async () => {
            await addCoordrole()
            scrollToBottom()
          }}
          >
          Add role
        </button>
      </div> -->

      <!-- Editing type select dropdown with options only me, anyone, and no one -->
      <!-- <div style="margin-bottom: 16px; margin-top: 16px; text-align: left;">
        Who can edit this {agreementType}:
        <select id="editing-type" name="editing-type" on:input={e => { editingType = e.target.value;} } required>
          <option value="only me">Only me</option>
          <option value="anyone">Anyone</option>
          <option value="no one">No one</option>
        </select>
      </div> -->

      <!-- post to bulletin? -->
      <!-- <div style="margin-bottom: 16px; text-align: left;">
        <input type="checkbox" id="post-to-bulletin" name="post-to-bulletin" checked={true} on:input={e => { postToBulletin = e.target.checked;} } required>
        <label for="post-to-bulletin">Post to bulletin</label>
      </div> -->

      <!-- invite specific people -->
      <!-- <div style="margin-bottom: 16px; text-align: left;">
        Invite specific people
        <input />
      </div> 
      <br> -->
      <mwc-button 
        raised
        label="Propose {agreementType}"
        disabled={!isCoordinationValid || !areCoordRolesValid}
        on:click={() => createCoordination()}
      ></mwc-button>
    
    {/if}

  
  <!-- <button on:click={() => {addToNotifiers()}}>.</button> -->

  <style>
    .optional-fields {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
    }

    .role-outer {
      display: flex;
      flex-direction: row;
      justify-content: space-between;
      border: 1px solid #ccc;
      border-radius: 4px;
      padding: 8px;
      margin-bottom: 8px;
      width: calc(100% - 28px);
      /* max-width: 400px; */
    }

    :global(.optional-button) {
      cursor: pointer;
      border: 0;
      background-color: transparent;
      font-weight: bold;
    }

    :global(.optional-button:hover) {
      text-decoration: underline;
    }

    .delete {
      border: 0;
      padding: 4px 6px;
      border-radius: 6px;
      margin-top: 2px;
      width: fit-content;
      height: fit-content;
      margin-left: 6px;
      cursor: pointer;
      display: flex;
      flex-direction: row;
    }

    .choose-type {
      display: flex;
      flex-direction: row;
      justify-content: space-around;
      /* margin-top: 16px; */
    }

    .choose-type > div {
      cursor: pointer;
      padding: 8px;
      border: 1px solid #ccc;
      border-radius: 4px;
      width: 100%;
      margin: 0;
      margin-bottom: 16px;
      /* margin: 0; */
      padding: 20px;
    }

    .choose-type > div:hover {
      background-color: #f0f0f0;
    }

    /* mwc-textfield {
      width: calc(100% - 20px);
    } */

    .title-input {
      width: calc(100% - 20px);
      height: 30px;
      padding: 8px 14px;
      background: #D5DAE540;
      border: 0;
      outline: 0;
      border-radius: 4px;
    }

    .description-input {
      width: calc(100% - 20px);
      height: 100px;
      padding: 14px;
      margin-top: 0.2em;
      margin-bottom: 1em;
      background: #D5DAE540;
      border: 0;
      border-radius: 4px;
    }

    .role {
      display: flex;
    }

    .add-role {
      height: 50px;
      border: 0;
      border-radius: 4px;
      margin: 4px;
      background-color: #608bdb;
      color: white;
      font-weight: bold;
      width: 100px;
      cursor: pointer;
    }

    .add-role:hover {
      background-color: #9ab8ee;
    }

    .add-role-button {
      cursor: pointer;
    }

    .add-role-button:hover {
      text-decoration: underline;
    }

    mwc-button {
      --mdc-theme-primary: #3360b3; /* Change this to your desired color */
    }

    .optional-field {
      margin-bottom: 16px; 
      text-align: left;
      padding: 8px;
      margin: 0.2em 0;
      border-radius: 4px;
      background: #D5DAE540;
    }
  </style>