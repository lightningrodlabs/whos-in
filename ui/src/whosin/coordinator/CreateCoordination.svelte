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
  
  import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
      import CreateCoordrole from './CreateCoordrole.svelte';
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  const dispatch = createEventDispatcher();
  
  
  let title: string | undefined;
  let description: string | undefined;
  let happeningDate: number | undefined;
  let signupDeadline: number | undefined;
  let reminderDate: number | undefined;
  let coordRoles: Coordrole[] = [];
  let roleTitle: string | undefined;
  let roleDescription: string | undefined;
  let minimum: number | undefined;
  let maximum: number | undefined;
  
  let errorSnackbar: Snackbar;
  
  $: title, description, happeningDate, signupDeadline, reminderDate, coordRoles, roleTitle, roleDescription, minimum, maximum;
  $: isCoordinationValid = true && title !== undefined && description !== undefined && coordRoles.length > 0; //&& happeningDate !== undefined && signupDeadline !== undefined && reminderDate !== undefined;//
  $: isCoordRoleValid = roleTitle != undefined && roleDescription != undefined && minimum != undefined && maximum != undefined && minimum <= maximum && minimum > 0;
  
  async function createCoordination() {
    const coordinationEntry: Coordination = {
      title: title!,
      description: description!,
      happening_date: happeningDate!,
      signup_deadline: signupDeadline!,
      reminder_date: reminderDate!,
      coordroles: coordRoles!,
    };
    
    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'create_coordination',
        payload: coordinationEntry  //{
          // coordination: coordinationEntry, 
          // coordroles: coordRoles
        // },
      });
  
      dispatch('coordination-created', { 
        coordinationHash: record.signed_action.hashed.hash 
      });
  
      // notify("")
      navigate("coordination", record.signed_action.hashed.hash);
  
    } catch (e) {
      errorSnackbar.labelText = `Error creating the coordination: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
  
  async function addCoordrole(roleTitle, roleDescription, min, max) {
    coordRoles.push({title: roleTitle, description: roleDescription, minimum: min, maximum: max});
    
    coordRoles = coordRoles;
  }
  
  async function remove_role(role:Coordrole) {
    let index = coordRoles.indexOf(role)
    coordRoles.splice(index, 1)
    coordRoles = coordRoles
  }
  
  </script>
  
  <mwc-snackbar bind:this={errorSnackbar} leading>
  </mwc-snackbar>
  
  <div class="white-container" style="display: flex; flex-direction: column">
    <h1 style="font-size: 24px; font-weight: 400; text-align: left;">Start an action</h1>
  
    <div style="margin-bottom: 16px; text-align: left;">
      <mwc-textfield label="Title"  on:input={e => { title = e.target.value; } } required></mwc-textfield>          
    </div>
              
    <div style="margin-bottom: 16px; text-align: left;">
      <mwc-textarea label="Description"  on:input={e => { description = e.target.value;} } required></mwc-textarea>          
    </div>
  
    <!-- <div class="dates">      
      <div style="margin-bottom: 16px; text-align: left;">
        <vaadin-date-time-picker label="Happening Date"  on:change={e => { happeningDate = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
      </div>
                
      <div style="margin-bottom: 16px; text-align: left;">
        <vaadin-date-time-picker label="Signup Deadline"  on:change={e => { signupDeadline = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
      </div>
                
      <div style="margin-bottom: 16px; text-align: left;">
        <vaadin-date-time-picker label="Reminder Date"  on:change={e => { reminderDate = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
      </div>
    </div> -->
  
    <div style="display: flex; flex-direction: column">
      <h2>Roles</h2>
  
      <div id="created-roles">
      {#each coordRoles as role}
      <div class="created-role">
        <strong>{role.title}</strong>
        <br>
        <div>Min: {role.minimum} Max: {role.maximum}</div>
        <br>
        <div>{role.description}</div>
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
    
      <div class="role">
        <mwc-textfield style="width: 20%" label="Title" on:input={e => { roleTitle = e.target.value; } } required></mwc-textfield>          
        <mwc-textfield style="width: 40%" label="Description"  on:input={e => { roleDescription = e.target.value;} } required></mwc-textfield>          
        <mwc-textfield style="width: 10%" type="number" label="Min"  on:input={e => { minimum = Number(e.target.value);} } required></mwc-textfield>          
        <mwc-textfield style="width: 10%" type="number" label="Max"  on:input={e => { maximum = Number(e.target.value);} } required></mwc-textfield>          
      </div>
  
    </div>
  
    <br>
    <mwc-button class="add-role"
    raised
    label="Add Role"
    disabled={!isCoordRoleValid}
    on:click={() => addCoordrole(roleTitle, roleDescription, minimum, maximum)}
    >
    </mwc-button>  
    <br>
    <mwc-button 
      raised
      label="Create Coordination"
      disabled={!isCoordinationValid}
      on:click={() => createCoordination()}
    ></mwc-button>
  </div>
  