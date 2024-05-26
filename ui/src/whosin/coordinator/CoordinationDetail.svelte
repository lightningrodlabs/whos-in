<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import { decode } from '@msgpack/msgpack';
  import type { Record, ActionHash, AppClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Coordination, Coordrole } from './types';
  import '@material/mwc-circular-progress';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-snackbar';
  import '@material/mwc-icon-button';
  import { navigate } from '../../store.js';
  import AttachmentsList from '../../AttachmentsList.svelte';
  import { isWeContext, type WAL } from '@lightningrodlabs/we-applet';
  import SvgIcon from '../../SvgIcon.svelte';
  import Avatar from './Avatar.svelte';
  import { getMyDna } from '../../util';
  import { countViewed, addToViewed, add_notification, weClientStored } from '../../store.js';

  const dispatch = createEventDispatcher();
  
  export let coordinationHash: ActionHash;
  
  let client: AppClient = (getContext(clientContext) as any).getClient();
  
  let loading = true;
  let error: any = undefined;
  
  let attachments = [];
  // let record: Record | undefined;
  let coordination: Coordination | undefined;
  let coordRoles; //: Coordrole[] | undefined;
  let sponsors;
  let committingInProcess = {};
  let weClient;
  weClientStored.subscribe(value => {
    weClient = value;
  });
  let dnaHash;
  
  let totalMin = 0;
  let totalUnderMin = 0;
  let totalParticipants = 0;
  let stringStartDate;
  let stringEndDate;
  let stringExpiresDate;

  let coordination_type;
  const coordination_type_icons = {
    "Event": "faCalendar",
    "Project": "faTask",
    "Agreement": "faAgreement"
  }
  const coordination_type_colors = {
    "Event": "357cff",
    "Project": "#ff951d",
    "Agreement": "#5301ae"
  }
  
  let errorSnackbar: Snackbar;
    
  $: error, loading, coordination, sponsors, committingInProcess;
  
  // onMount(() => fetchCoordination());
  // onMount(() => fetchRoles());
  
  onMount(async () => {
    dnaHash = await getMyDna("whosin", client)
    await fetchCoordination()
    // .then(() => {
    await fetchRoles()
    // })
    addToViewed(coordinationHash, client)

    getSponsors()
  });


  const copyWalToPocket = () => {
    const attachment: WAL = { hrl: [dnaHash, coordinationHash], context: "" }
    weClient?.walToPocket(attachment)
  }
  
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
        attachments = coordination.attachments?.map((attachment) => {
          return {
            hrl: JSON.parse(attachment.hrl),
            context: attachment.context
          }
        })
        let options: Intl.DateTimeFormatOptions = { 
          weekday: 'long',
          year: 'numeric', 
          month: 'long',
          day: 'numeric', 
          hour: 'numeric', 
          minute: 'numeric', 
          hour12: true 
        };
        coordination_type = coordination.coordination_type
        stringStartDate = new Date(coordination.starts_date / 1000).toLocaleDateString(undefined, options);
        // if year is current year, don't show year
        stringStartDate = stringStartDate.replace(" " + new Date().getFullYear() + " ", " ");
        stringEndDate = new Date(coordination.ends_date / 1000).toLocaleDateString(undefined, options).replace(" " + new Date().getFullYear() + " ", " ");
        stringExpiresDate = new Date(coordination.signup_deadline / 1000).toLocaleDateString(undefined, options).replace(" " + new Date().getFullYear() + " ", " ");
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
        record.forEach(r => {
          let min = decode(r.coordrole.entry.Present.entry)["minimum"];
          let underMin = Math.min(r.participants, min);
          totalParticipants += r.participants;
          totalMin += min;
          totalUnderMin += underMin;
          totalMin = totalMin;
          totalUnderMin = totalUnderMin;
        })
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
  
  async function commitMe(coordRoleHash, coordroleTimestamp) {
    // console.log(coordRoleHash)
    // let coordRoleHash = coordRole;
    committingInProcess[JSON.stringify(coordRoleHash)] = true;
    
    try {
      await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'commit_to_coordrole',
        payload: coordRoleHash,
      });
      totalParticipants += 1;
      totalUnderMin += 1;
      // dispatch('coordrole-committed', { coo: coordinationHash });
      // navigate("coordination", coordinationHash);
      await fetchRoles();
      // navigate("all-coordinations", {})
      // coordRole.committed = true;
      if (totalUnderMin >= totalMin) {
        add_notification({
          "timestamp": coordroleTimestamp,
          "type": "coordination-activation",
          "description": "The " + Object.keys(coordination.coordination_type)[0].toLocaleLowerCase() + " " + coordination.title + " has reached minimum participation",
          "hash": coordinationHash,
          "seen": false,
        })
      }
      committingInProcess[JSON.stringify(coordRoleHash)] = false;
    } catch (e: any) {
      await fetchRoles();
      // coordRole.committed = true;
      committingInProcess[JSON.stringify(coordRoleHash)] = false;
      console.log(e)
      errorSnackbar.labelText = `Error commiting to the coordination: ${e}`;
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
  

  <div style="display: flex; flex-direction: row; margin-bottom: 0px; justify-content: space-between">
        <div style="display: flex; flex-direction: row; margin-bottom: 0px">
          <h1 style="margin: 0; font-weight: bold;">
            { coordination.title }
          </h1>
          <!-- <div class="type-label" style="background: {coordination_type_colors[coordination_type]}">
            <SvgIcon color="#fff" size=15 icon="{coordination_type_icons[coordination_type]}" /> 
            <div style="margin: 2px;">
              {coordination_type}
            </div>
          </div> -->
        </div>
          
        <div style="display: flex; flex-direction: row; margin-bottom: 0px">
          <div class="type-label" style="background: {coordination_type_colors[coordination_type]};     margin-top: 0;
          margin-bottom: 10px;
          margin-right: 8px;
          padding: 3px 10px;
          font-weight: 100;
          font-size: 14px;">
            <SvgIcon style="height: 0" color="#fff" size=15 icon="{coordination_type_icons[coordination_type]}" /> 
            <div style="margin: 2px; height: 0;">
              {coordination_type}
            </div>
          </div>
          <div class="status-label">
            <!-- active, happening today, expired, gathering participation -->
            {#if coordination.ends_date && coordination.ends_date < (new Date().getTime() * 1000)}
              <div style="background: #ff0000; color: #fff; padding: 3px 5px; border-radius: 5px; margin-right: 10px;">
                Expired
              </div>
            {:else if totalUnderMin >= totalMin && coordination.starts_date && coordination.starts_date < (new Date().getTime() * 1000)}
              <div style="background: #cd1dff; color: #fff; padding: 3px 5px; border-radius: 5px; margin-right: 10px;">
                Happening today
              </div>
            {:else if totalMin > 0 && totalUnderMin < totalMin}
              <div style="background: #ff951d; color: #fff; padding: 3px 5px; border-radius: 5px; margin-right: 10px;">
                Gathering participation
              </div>
            {:else if totalUnderMin >= totalMin}
              <div style="background: #57ca01; color: #fff; padding: 3px 5px; border-radius: 5px; margin-right: 10px;">
                Active
              </div>
            {/if}
          </div>
          <button title="Add Board to Pocket" class="attachment-button" style="margin-right:10px; cursor: pointer; margin-right: 10px;
          cursor: pointer;
          height: 24px;
          width: 40px;
          padding: 0px;" on:click={()=>copyWalToPocket()} >          
            <SvgIcon icon="addToPocket" size="20px"/>
          </button>
        </div>
    </div>
  
    {#if isWeContext}
      <div style="display: flex; flex-direction: row; margin-bottom: 5px">
        <AttachmentsList {attachments} allowDelete={false}/>
      </div>
    {/if}
  
    <!-- <div style="display: flex; flex-direction: row; margin-bottom: 0px">
      <SvgIcon size=16 icon="{coordination_type_icons[coordination_type]}" /> 
      <div style="margin: 2px;">
        {coordination_type}
      </div>
    </div> -->

    {#if coordination.signup_deadline}
      <div class="action-details">
        <!-- deadline to signup -->
        <div class="action-date">
          <SvgIcon color="#484848" size=16 icon="faClock" />
          Deadline to signup: <span style="white-space: pre-line">{ stringExpiresDate }</span>
        </div>
      </div>
    {/if}

    {#if coordination.starts_date}
      <div class="action-details">
        <!-- if start date and end date are on the same day -->
        {#if stringStartDate.split(',')[0] == stringEndDate.split(',')[0]}
          <div class="action-date">
            <SvgIcon color="#484848" size=16 icon="faClock" />
            Event date: <span style="white-space: pre-line">{ stringStartDate.split(" at")[0] } {stringStartDate.split('at ')[1]} to {stringEndDate.split('at ')[1]}</span>
          </div>
        {:else}
          <div class="action-date">
            <SvgIcon color="#484848" size=16 icon="faClock" />
            Event dates: <span style="white-space: pre-line">{ stringStartDate }</span>
            to <span style="white-space: pre-line">{ stringEndDate }</span>
          </div>
        {/if}
      </div>
    {:else if coordination.ends_date}
      <div class="action-date">
        <SvgIcon color="#484848" size=16 icon="faClock" />
        Deadline to complete: <span style="white-space: pre-line">{ stringEndDate }</span>
      </div>
    {/if}

    <div style="display: flex; flex-direction: row; margin-bottom: 0; margin-top: 10px;">
      <span class="action-description" style="white-space: pre-line">{ coordination.description }</span>
    </div>

    <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      Ends on: &nbsp;<span style="white-space: pre-line">{ new Date(coordination.ends_date / 1000).toLocaleString() }</span>
    </div>

    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      Signup deadline: &nbsp;<span style="white-space: pre-line">{ new Date(coordination.signup_deadline / 1000).toLocaleString() }</span>
    </div> -->
  
    <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      Reminder on: <span style="white-space: pre-line">{ new Date(coordination.reminder_date / 1000).toLocaleString() }</span>
    </div> -->
  
    <h2 style="
      float: left;
      width: 10px;
      font-weight: bold;
      font-size: 18px;
    ">Roles</h2>
  
    {#if coordRoles}
      <div style="display:flex; flex-wrap: wrap;">
        {#each coordRoles as role}
          <div class="role-outer">
            <div class="coordrole-details-section">
                
              <div class="role-item" style="margin-bottom: 6px;">
                <strong>{decode(role.coordrole.entry.Present.entry)["title"]}</strong>
              </div>
        
              <div class="role-item">
                {decode(role.coordrole.entry.Present.entry)["description"]}
              </div>
        
              <div class="progress-extraouter">
                <div class="participation-meter participation-meter-outer">
                  <div class="participation-progress" style="
                  width: {decode(role.coordrole.entry.Present.entry)["minimum"] > 0 ? (role.participants/decode(role.coordrole.entry.Present.entry)["minimum"] * 100) : 100}%;
                  background-color: rgba(255, 196, 0, 0.75);">
                    &nbsp;
                  </div>
                </div>
                <div class="role-item" style="display: flex; flex-direction: row; color: #a5a5a5; margin: 4px; padding: 7px; font-weight: 600; font-size: 12px;">
                  {decode(role.coordrole.entry.Present.entry)["minimum"] > 0 ? (Math.round(role.participants/decode(role.coordrole.entry.Present.entry)["minimum"] * 100)) : 100}%
                </div>


                <!-- <div class="role-item" style="margin-top: 8px; display: flex; flex-direction: row;">
                    <div style="display: flex; width: max-content; margin: 0 4px;">
                      {role.participants} committed
                    </div>
                  {#if role.participants < decode(role.coordrole.entry.Present.entry)["minimum"]}
                    <div style="display: flex; width: max-content; margin: 0 4px;">
                      and {decode(role.coordrole.entry.Present.entry)["minimum"] - role.participants} more needed
                    </div>
                  {:else}
                    <span>
                      ✔
                    </span>
                  {/if}
                </div> -->
              </div>
        
              <!-- {Object.is(role.participants[0], client.myPubKey)} -->
              <!-- {JSON.stringify(role.participants/decode(role.coordrole.entry.Present.entry)["minimum"] * 100)} -->
              <!-- {JSON.stringify(role.committed)} -->
              <!-- {JSON.stringify(role.participants.includes(client.myPubKey))} -->
              
              
              </div>
        
            <div class="action-section" style="padding: 0 10px; display: flex; justify-content: space-between;">
              <!-- <div class="role-item" style="margin-top: 8px;">{role.participants} committed
                {#if role.participants < decode(role.coordrole.entry.Present.entry)["minimum"]}
                  and {decode(role.coordrole.entry.Present.entry)["minimum"] - role.participants} more needed
                {:else}
                  ✔
                {/if}
              </div> -->

              <!-- {JSON.stringify(decode(role.coordrole.entry.Present.entry))} -->
              <!-- {JSON.stringify(role.participants_details)} -->

              <div class="role-item" style="margin-top: 8px; display: flex; flex-direction: row;">

                {#each role.participants_details.slice(0, 5) as participant}
                  <div class="role-item" style="margin-left: -13px">
                    <Avatar showNickname={false} agentPubKey={participant}  size={24} namePosition="row"></Avatar>
                  </div>
                {/each}
                {#if role.participants_details.length > 5}
                  <div class="role-item">
                    + {role.participants_details.length - 5}
                  </div>
                {/if}

                <div style="display: flex; width: max-content; margin: 4px; color: #5e5e5e;">
                  {role.participants} committed
                </div>
                {#if role.participants < decode(role.coordrole.entry.Present.entry)["minimum"]}
                  <div style="display: flex; width: max-content; margin: 4px; color: #5e5e5e;">
                    • {decode(role.coordrole.entry.Present.entry)["minimum"] - role.participants} more needed
                  </div>
                {:else}
                  <span style="margin: 4px; color: #5e5e5e;">
                    ✔
                  </span>
                {/if}
              </div>

              {#if committingInProcess[JSON.stringify(role.coordrole.signed_action.hashed.hash)]}
                <div class="commit" style="padding: 0; height: fit-content">
                  <mwc-circular-progress indeterminate></mwc-circular-progress>
                </div>

              <!-- not past the signup deadeline and not past the end date -->
              {:else if (!coordination.signup_deadline || coordination.signup_deadline > (new Date().getTime() * 1000)) && (!coordination.ends_date || coordination.ends_date > (new Date().getTime() * 1000))}
                {#if role.committed}
                  <button class="commit" on:click={() => unCommitMe(role.coordrole.signed_action.hashed.hash)} >Remove me</button>
                {:else if role.participants < decode(role.coordrole.entry.Present.entry)["maximum"]}
                  <button class="commit" on:click={() => commitMe(role.coordrole.signed_action.hashed.hash, role.coordrole.signed_action.hashed.content.timestamp)} >Add me</button>
                {/if}
              {/if}
            </div>
        
          </div>
        {/each}
      </div>
    {/if}

    {#if sponsors && sponsors.length && sponsors.length == 1 && sponsors.includes(client.myPubKey.join())}
      <p class="notice">Since no one else has signed up for a role yet, you can still hide this {coordination_type.toLowerCase()} from the front page. <button id="hide" on:click={() => unsponsor()}>Hide</button></p>
    {/if}
  
  <div class="invisible" on:click={() => markSpam()}>.</div>
  {/if}