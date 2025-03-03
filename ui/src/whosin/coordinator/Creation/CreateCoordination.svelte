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
  import { secondsToDateInput } from './helper';
  export let agreementType: string = "event";
  import CreateRole from './CreateRole.svelte';

  let client: AppClient = (getContext(clientContext) as any).getClient();
  
  let weClient: any;
  weClientStored.subscribe(value => {
    weClient = value;
  });

  const dispatch = createEventDispatcher();
  
  let showDescription = false;
  let attachmentsDialog : AttachmentsDialog
  let attachments: Array<WALUrl> = [];
  let title: string | undefined;
  let description: string | undefined = '';
  export let startsDate: number | undefined = agreementType == "event" ? new Date().valueOf() * 1000 : undefined;
  export let endsDate: number | undefined = agreementType == "event" ? new Date().valueOf() * 1000 + 3600 : undefined;
  let signupDeadline: number | undefined;
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
  $: isCoordinationValid = title !== undefined && description !== undefined && coordRoles.length > 0 && (agreementType != "event" || (startsDate != undefined)) //&& happeningDate !== undefined && signupDeadline !== undefined && reminderDate !== undefined;//
  $: isCoordRoleValid = roleTitle != undefined && roleDescription != undefined && minimum != undefined && maximum != undefined && minimum <= maximum && minimum >= 0;
  
  async function createCoordination() {
    const coordinationEntry: Coordination = {
      title: title!,
      description: description!,
      // capitalized agreementType
      coordination_type: agreementType!,
      starts_date: startsDate,
      ends_date: endsDate,
      signup_deadline: signupDeadline,
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
      console.log(weClient, weClient.renderInfo)
      try {
        weClient.renderInfo.view.resolve(wal)
      } catch (e) {
        console.log("Created coordination")
      }
      navigate("coordination", record.signed_action.hashed.hash);
  
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
    coordRoles.push({title: roleTitle, description: roleDescription, minimum: minimum, maximum: maximum});
    roleTitle = undefined;
    roleDescription = undefined;
    minimum = undefined;
    maximum = undefined;
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
    dnaHash = await getMyDna("whosin", client);
    titleField.focus();
    if (agreementType == "event") {
      // startsDate = new Date().valueOf() * 1000;
      // endsDate = new Date().valueOf() * 1000 + 3600;
      coordRoles.push({title: "Participant", description: "", minimum: 1, maximum: 100});
      coordRoles = coordRoles;
    } else if (agreementType == "project") {
      coordRoles.push({title: "Participant", description: "", minimum: 1, maximum: 100});
      coordRoles = coordRoles;
    } else if (agreementType == "agreement") {
      coordRoles.push({title: "Signatory", description: "", minimum: 1, maximum: 100});
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
    
      <h3 style="text-transform: capitalize;">New {agreementType}</h3>
      <div style="margin-bottom: 16px; text-align: left;">
        <input class="title-input" placeholder="Title" bind:this={titleField} on:input={e => { title = e.target.value; } } />
      </div>
      
      <div class="optional-fields">
        <div class="dates">
          {#if agreementType == "event"}
            <div style="margin-bottom: 16px; text-align: left; flex-grow: 1;">
              <!-- datetime -->
              <span style="text-transform: capitalize">
                {agreementType}
              </span> starts
              <input type="datetime-local" id="start-date" name="start-date" 
              value={secondsToDateInput(startsDate) || ""}
              on:input={e => { 
                startsDate = new Date(e.target.value).valueOf() * 1000;
                // console.log("endsDate", endsDate)
                // endsDate ? null : endsDate = new Date(startsDate + 3600).valueOf() * 1000;
                // console.log("endsDate", endsDate)
              } } required>

              <span style="font-weight: 300; font-size: 14px;">
                {!endsDate ? "(all-day event)" : ""}
              </span>
              <!-- {JSON.stringify(secondsToDateInput(startsDate))}hi -->
      
              <!-- <vaadin-date-time-picker label="Starts"  on:change={e => { startsDate = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>           -->
            </div>

            {#if !endsDate}
              <br>
              <div style="margin-bottom: 16px; text-align: left;">
                <button class="optional-button" on:click={() => endsDate = new Date().valueOf() * 1000}>
                + end date
                </button>
              </div>
              {:else}
              <div style="margin-bottom: 16px; text-align: left;">
                <span style="text-transform: capitalize">
                {agreementType}
                </span> ends
                <input type="datetime-local" id="end-date" name="end-date"
                value={endsDate ? secondsToDateInput(endsDate) : ""}
                on:input={e => { 
                  let newEndsDate = new Date(e.target.value).valueOf() * 1000;
                  if (newEndsDate >= startsDate) {
                  endsDate = newEndsDate;
                  } else {
                  e.target.value = null
                  }
                } } required>
                <button class="optional-button" on:click={() => endsDate = undefined}>
                Remove end date
                </button>
              </div>
              {/if}
                <!-- {JSON.stringify(secondsToDateInput(endsDate))}ho -->
            <!-- </div> -->
          {:else if agreementType == "project"}
          <div style="margin-bottom: 16px; text-align: left;">
            Deadline to complete (optional)
            <input type="datetime-local" id="end-date" name="end-date"
              value={endsDate ? new Date(endsDate).toISOString().slice(0, 16) : ""}
              on:input={e => { 
                let newEndsDate = new Date(e.target.value).valueOf() * 1000;
                if (!startsDate || newEndsDate <= startsDate) {
                  endsDate = newEndsDate;
                } else {
                  e.target.value = null
                }
              } } required>
          </div>
          {/if}     

          <!-- <div style="margin-bottom: 16px; text-align: left;">
            Reminder
            <input type="datetime-local" id="reminder-date" name="reminder-date" on:input={e => { reminderDate = new Date(e.target.value).valueOf() * 1000;} } required>
          </div> -->
        </div>

        <div class="dates">
          {#if !signupDeadline}
            <div style="margin-bottom: 16px; text-align: left;">
              <button class="optional-button" on:click={() => signupDeadline = new Date().valueOf() * 1000}>
                + deadline to sign up
              </button>
            </div>
          {:else}
            <div style="margin-bottom: 16px; text-align: left;">
              Deadline to signup
              <input type="datetime-local" id="signup-deadline" name="signup-deadline" 
                value={signupDeadline ? new Date(signupDeadline).toISOString().slice(0, 16) : ""}
                on:input={e => {
                  signupDeadline = new Date(e.target.value).valueOf() * 1000;
                }} required>
                <!-- remove? -->
                <button class="optional-button" on:click={() => signupDeadline = undefined}>
                  Remove deadline
                </button>
            </div>
          {/if}
        </div>


        {#if !showDescription}
        <!-- + description -->
        <div style="margin-bottom: 16px; text-align: left;">
          <button class="optional-button" on:click={() => showDescription = true}>+ description</button>
        </div>
        {:else}
          <div style="margin-bottom: 16px; text-align: left;">
            <h3 style="text-transform: capitalize;">{agreementType} Description</h3>
            <textarea class="description-input" placeholder="Type Here"  on:input={e => { description = e.target.value;} } />
            <button class="optional-button" on:click={() => showDescription = false}>Remove description</button>
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

      <div style="display: flex; flex-direction: column">
        <h2>Roles for this {agreementType}</h2>
    
        <div id="created-roles">
          {#each coordRoles as role}
          <div class="role-outer">
            <CreateRole {role} />
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
              <button class="delete" on:click={() => removeRole(role)}>Remove</button>
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
      <div style="margin-bottom: 16px; text-align: left;">
        Who can edit this {agreementType}:
        <select id="editing-type" name="editing-type" on:input={e => { editingType = e.target.value;} } required>
          <option value="only me">Only me</option>
          <option value="anyone">Anyone</option>
          <option value="no one">No one</option>
        </select>
      </div>

      <!-- post to bulletin? -->
      <div style="margin-bottom: 16px; text-align: left;">
        <input type="checkbox" id="post-to-bulletin" name="post-to-bulletin" checked={true} on:input={e => { postToBulletin = e.target.checked;} } required>
        Post to bulletin
      </div>

      <!-- invite specific people -->
      <div style="margin-bottom: 16px; text-align: left;">
        Invite specific people
        <input />
      </div>

      <br>
      <p class="notice">Warning: After proposing an {agreementType}, it belongs to everyone and cannot be edited or deleted.</p>
      <mwc-button 
        raised
        label="Propose {agreementType}"
        disabled={!isCoordinationValid}
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
      border-radius: 4px;
    }

    .description-input {
      width: calc(100% - 20px);
      height: 100px;
      padding: 14px;
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
  </style>