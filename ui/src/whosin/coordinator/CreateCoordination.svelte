<script lang="ts">
  import { createEventDispatcher, getContext } from 'svelte';
  import { type AppAgentClient, type Record } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Coordination, Coordrole } from './types';
  import '@material/mwc-button';
  import '@material/mwc-snackbar';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-textfield';
  import '@material/mwc-textarea';
  import { view, viewHash, navigate } from '../../store.js';
  import AttachmentsDialog from '../../AttachmentsDialog.svelte';
  import { isWeContext } from '@lightningrodlabs/we-applet';
  import { countViewed, addToViewed } from '../../store.js';
  import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
  import SvgIcon from './SvgIcon.svelte';
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  const dispatch = createEventDispatcher();
  
  
  let attachmentsDialog : AttachmentsDialog
  let attachments = []
  let title: string | undefined;
  let description: string | undefined = '';
  let startsDate: number | undefined;
  let endsDate: number | undefined;
  let signupDeadline: number | undefined;
  let reminderDate: number | undefined;
  let coordRoles: Coordrole[] = [];
  let roleTitle: string | undefined;
  let roleDescription: string | undefined;
  let minimum: number | undefined;
  let maximum: number | undefined;
  let notifierVisibility = false;
  let agreementType: string | undefined;
  const agreementTypeGrammar = {
    "event": "an event",
    "project": "a project",
    "ongoing agreement": "an ongoing agreement"
  }
  
  let errorSnackbar: Snackbar;
  
  $: title, description, startsDate, endsDate, signupDeadline, reminderDate, coordRoles, roleTitle, roleDescription, minimum, maximum, attachments;
  $: isCoordinationValid = title !== undefined && description !== undefined && coordRoles.length > 0 && (agreementType != "event" || (startsDate !== undefined && endsDate != undefined)) //&& happeningDate !== undefined && signupDeadline !== undefined && reminderDate !== undefined;//
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
      attachments: attachments.map(a => {
        return {
          hrl: JSON.stringify(a.hrl),
          context: a.context
        }
      }),
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
  
  async function remove_role(role:Coordrole) {
    let index = coordRoles.indexOf(role)
    coordRoles.splice(index, 1)
    coordRoles = coordRoles
  }
  
  </script>
  
  <mwc-snackbar bind:this={errorSnackbar} leading>
  </mwc-snackbar>
  
  <div class="white-container" style="display: flex; flex-direction: column; margin-top: 30px;">

    <!-- {:else} -->
      <!-- {#if !agreementType} -->
        <h3>
          Coordination type
        </h3>
      <!-- {/if} -->
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

    {#if agreementType}

      <!-- <h1 style="font-size: 24px; font-weight: 400; text-align: left;">Create {agreementTypeGrammar[agreementType]}</h1> -->
    
      <div style="margin-bottom: 16px; text-align: left;">
        <h3 style="text-transform: capitalize;">{agreementType} Title</h3>
        <input class="title-input" placeholder="Type Here"  on:input={e => { title = e.target.value; } } />
      </div>
                
      <div style="margin-bottom: 16px; text-align: left;">
        <h3 style="text-transform: capitalize;">{agreementType} Description</h3>
        <textarea class="description-input" placeholder="Type Here"  on:input={e => { description = e.target.value;} } />
      </div>

      {#if isWeContext}
        <div style="display:flex; flex-wrap:wrap; align-items: center; margin-bottom:10px;">
          <h2>Attachments &nbsp;

          </h2>
        <AttachmentsDialog bind:this={attachmentsDialog} bind:attachments on:add-attachments={
          (e) => {
            console.log("add-attachments", e.detail)
            attachments = e.detail.attachments
          }
        }></AttachmentsDialog>
        </div>
      {/if}
    
      <div class="dates">
        <div style="margin-bottom: 16px; text-align: left;">
          Deadline to signup
          <input type="datetime-local" id="signup-deadline" name="signup-deadline" 
            on:input={e => {
              signupDeadline = new Date(e.target.value).valueOf() * 1000;}
            } required>
          <!-- <vaadin-date-time-picker label="Signup Deadline"  on:change={e => { signupDeadline = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>           -->
        </div>

      </div>
      <div class="dates">

        {#if agreementType == "event"}
          <div style="margin-bottom: 16px; text-align: left;">
            <!-- datetime -->
            <span style="text-transform: capitalize">
              {agreementType}
            </span> starts
            <input type="datetime-local" id="start-date" name="start-date" on:input={e => { startsDate = new Date(e.target.value).valueOf() * 1000;} } required>
            <!-- <vaadin-date-time-picker label="Starts"  on:change={e => { startsDate = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>           -->
          </div>

          <div style="margin-bottom: 16px; text-align: left;">
            <span style="text-transform: capitalize">
              {agreementType}
            </span> ends
            <input type="datetime-local" id="end-date" name="end-date"
              on:input={e => { 
                let newEndsDate = new Date(e.target.value).valueOf() * 1000;
                if (newEndsDate >= startsDate) {
                  endsDate = newEndsDate;
                } else {
                  e.target.value = null
                }
              } } required>
          </div>
        {:else if agreementType == "project"}
        <div style="margin-bottom: 16px; text-align: left;">
          Deadline to complete
          <input type="datetime-local" id="end-date" name="end-date"
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
    
      <div style="display: flex; flex-direction: column">
        <h2>Roles</h2>
    
        <div id="created-roles">
          {#each coordRoles as role}
          <div class="role-outer">
            <strong>{role.title}</strong>
            <br>
            <div>{role.description}</div>
            <br>
            <div>Min: {role.minimum} Max: {role.maximum}</div>
            <br>
            <button class="delete" on:click={() => remove_role(role)}>Remove</button>
            <!-- <button class="delete" on:click={async () => {
              remove_role(role)
              return undefined;
            }}>Remove</button> -->
            
            <!-- <button class="delete" on:click={remove_role(role)}>Remove</button> -->
          </div>
          {/each}
          </div>
        </div>

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
        <!-- <mwc-textfield style="width: 40%" label="Description" id="role-description-field"  on:input={e => { roleDescription = e.target.value;} } required></mwc-textfield>           -->
        <!-- <mwc-textfield style="width: 10%" type="number" label="Min" id="minimum-field" on:input={e => { minimum = Number(e.target.value);} } required></mwc-textfield> -->
        <!-- <mwc-textfield style="width: 10%" type="number" label="Max" id="maximum-field"  on:input={e => { maximum = Number(e.target.value);} } required></mwc-textfield>           -->
      </div>

      <br>
      <p class="notice">Warning: After proposing a {agreementType}, it belongs to everyone and cannot be edited or deleted.</p>
      <mwc-button 
        raised
        label="Propose {agreementType}"
        disabled={!isCoordinationValid}
        on:click={() => createCoordination()}
      ></mwc-button>
    
    {/if}

  </div>
  
  <!-- <button on:click={() => {addToNotifiers()}}>.</button> -->

  <style>
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

    mwc-button {
      --mdc-theme-primary: #3360b3; /* Change this to your desired color */
    }
  </style>