<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import { decode } from '@msgpack/msgpack';
  import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Coordination, Coordrole } from './types';
  import '@material/mwc-circular-progress';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-snackbar';
  import '@material/mwc-icon-button';
  import { navigate } from '../../store.js';
  
  const dispatch = createEventDispatcher();
  
  export let coordinationHash: ActionHash;
  
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  let loading = true;
  let error: any = undefined;
  
  // let record: Record | undefined;
  let coordination: Coordination | undefined;
  let coordRoles; //: Coordrole[] | undefined;
  let sponsors;
  
  let errorSnackbar: Snackbar;
    
  $: error, loading, coordination, sponsors;
  
  // onMount(() => fetchCoordination());
  // onMount(() => fetchRoles());
  
  onMount(() => {
    fetchCoordination()
    .then(() => {
      fetchRoles()
    })

    getSponsors()
  });
  
  async function fetchCoordination() {
    loading = true;
    error = undefined;
    let record = undefined;
    coordination = undefined;
    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'get_coordination',
        payload: coordinationHash,
      });
      if (record) {
        coordination = decode((record.entry as any).Present.entry) as Coordination;
      }
    } catch (e) {
      error = e;
    }
  
    loading = false;
  }
  
  async function fetchRoles() {
    error = undefined;
    let record = undefined;
    coordRoles = undefined;
    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'get_coordroles_for_coordination',
        payload: coordinationHash,
      });
      if (record) {
        // console.log(record)
        coordRoles = record;//decode((record.entry as any).Present.entry) as Coordrole[];
      } else {
        console.log("?")
      }
    } catch (e) {
      error = e;
    }
  
    loading = false;
  }

  async function sponsor() {
    error = undefined;
    let record = undefined;
    coordination = undefined;
    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'add_sponsor_for_coordination',
        payload: coordinationHash,
      });
    } catch (e) {
      error = e;
    }
  }

  async function unsponsor() {
    let confirmation = window.confirm("Hide this post?")
    if (confirmation) {
      error = undefined;
      let record = undefined;
      
      try {
        record = await client.callZome({
          cap_secret: null,
          role_name: 'whosin',
          zome_name: 'coordinator',
          fn_name: 'remove_sponsor_for_coordination',
          payload: coordinationHash,
        });
        navigate("all-coordinations");
      } catch (e) {
        error = e;
      }
    }
  }

  async function getSponsors() {
    error = undefined;
    let record = undefined;
    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'get_sponsors_for_coordination',
        payload: coordinationHash,
      });
    } catch (e) {
      error = e;
    }
    if (record) {
      // sponsors = record;
      sponsors = [];
      record.forEach(element => {
        sponsors.push(element.join())
      });
    }
  }

  async function markSpam() {
    let confirmation = window.confirm("Report as spam?")
    if (confirmation) {
      error = undefined;
      let record = undefined;
      
      try {
        record = await client.callZome({
          cap_secret: null,
          role_name: 'whosin',
          zome_name: 'coordinator',
          fn_name: 'add_spam_reporter_for_coordination',
          payload: coordinationHash,
        });
      } catch (e) {
        error = e;
      }
    }
  }

  async function fetchCoordrole(coordroleCoded) {
    return decode((coordroleCoded.entry).Present.entry)
  }
  
  async function commitMe(coordRole) {
    let coordRoleHash = coordRole;
    
    try {
      await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'commit_to_coordrole',
        payload: coordRoleHash,
      });
      // dispatch('coordrole-committed', { coo: coordinationHash });
      // navigate("coordination", coordinationHash);
      fetchRoles();
      // navigate("all-coordinations", {})
      coordRole.committed = true;
    } catch (e: any) {
      errorSnackbar.labelText = `Error commiting to the coordination: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
  
  async function unCommitMe(coordRole) {
    let coordRoleHash = coordRole;
    try {
      await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'uncommit_to_coordrole',
        payload: coordRoleHash,
      });
      // dispatch('coordrole-committed', { coo: coordinationHash });
      // navigate("all-coordinations", {});
      fetchRoles();
      // navigate("coordination", coordinationHash);
      coordRole.committed = false;
    } catch (e: any) {
      errorSnackbar.labelText = `Error uncommitting: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
  </script>
  
  <mwc-snackbar bind:this={errorSnackbar} leading>
  </mwc-snackbar>
  
  {#if loading}
  <div style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  {:else if error}
  <span>Error fetching the coordination: {error.data.data}</span>
  {:else}
  
  <div class="white-container" style="display: flex; flex-direction: column">

    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <h1>{ coordination.title }</h1>
    </div>
  
    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span style="white-space: pre-line">{ coordination.description }</span>
    </div>
  
    <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      Happening on:&nbsp;<span style="white-space: pre-line">{ new Date(coordination.happening_date / 1000).toLocaleString() }</span>
    </div> -->
  
    <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      Deadline: &nbsp;<span style="white-space: pre-line">{ new Date(coordination.signup_deadline / 1000).toLocaleString() }</span>
    </div> -->
  
    <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      Reminder on: <span style="white-space: pre-line">{ new Date(coordination.reminder_date / 1000).toLocaleString() }</span>
    </div> -->
  
    <h2 style="float: left; width: 10px;">Roles</h2>
  
    {#if coordRoles}
    {#each coordRoles as role}
    <div class="role-outer">
      <div class="action-section">
          
        <div class="role-item">
          <strong>{decode(role.coordrole.entry.Present.entry)["title"]}</strong>
        </div>
  
        <div class="role-item">
          {decode(role.coordrole.entry.Present.entry)["description"]}
        </div>
  
        <div class="progress-extraouter">
          <div class="participation-meter participation-meter-outer">
            <div class="participation-progress" style="
            width: {role.participants/decode(role.coordrole.entry.Present.entry)["minimum"] * 100}%;
            background-color: rgba(255, 196, 0, 0.75);">
              &nbsp;
            </div>
          </div>
        </div>
  
        <!-- {Object.is(role.participants[0], client.myPubKey)} -->
        <!-- {JSON.stringify(role.participants/decode(role.coordrole.entry.Present.entry)["minimum"] * 100)} -->
        <!-- {JSON.stringify(role.committed)} -->
        <!-- {JSON.stringify(role.participants.includes(client.myPubKey))} -->
        
        {#if role.committed}
          <button class="commit" on:click={() => unCommitMe(role.coordrole.signed_action.hashed.hash)} >Remove me</button>
        {:else if role.participants < decode(role.coordrole.entry.Present.entry)["maximum"]}
          <button class="commit" on:click={() => commitMe(role.coordrole.signed_action.hashed.hash)} >Add me</button>
        {/if}
        </div>
  
      <div class="action-section">
        <div class="role-item">{role.participants} committed
          {#if role.participants < decode(role.coordrole.entry.Present.entry)["minimum"]}
            and {decode(role.coordrole.entry.Present.entry)["minimum"] - role.participants} more needed
          {:else}
            ✔
          {/if}
        </div>
      </div>
  
    </div>
    {/each}
    {/if}

    {#if sponsors && sponsors.length && sponsors.length == 1 && sponsors.includes(client.myPubKey.join())}
      <p>Since no one else has signed up for a role, you can still hide this action. <button on:click={() => unsponsor()}>Hide</button></p>
    {/if}
  
  <div class="invisible" on:click={() => markSpam()}>.</div>
  </div>
  {/if}
  
  