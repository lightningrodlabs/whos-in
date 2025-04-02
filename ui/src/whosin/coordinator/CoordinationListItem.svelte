<script lang="ts">
    import { createEventDispatcher, onMount, getContext } from 'svelte';
    import '@material/mwc-circular-progress';
    import { decode } from '@msgpack/msgpack';
    import type { Record, ActionHash, AppClient, EntryHash, AgentPubKey } from '@holochain/client';
    import { clientContext } from '../../contexts';
    import type { Coordination } from './types';
    import '@material/mwc-circular-progress';
    import type { Snackbar } from '@material/mwc-snackbar';
    import '@material/mwc-snackbar';
    import '@material/mwc-icon-button';
    import SvgIcon from '../../SvgIcon.svelte';
    import { view, viewHash, navigate, weClientStored } from '../../store.js';
    import { decodeHashFromBase64 } from '@holochain/client';
    import { WeaveClient } from '@lightningrodlabs/we-applet';
    import { appletHashFromAppId, getCoordinationLabel } from '../../util';
    import { getAppletInfoAndGroupsProfiles } from '@lightningrodlabs/we-elements';
    import { refetchCoordinationDetails } from '../../crud/refetch';
    import { allCoordinationsDetails } from '../../crud/dataStore';
    import { allCoordinations, myCoordinations } from '../../crud/dataStore';
    import { encodeHashToBase64 } from '@holochain/client';

    let weClient: WeaveClient;
    weClientStored.subscribe(value => {
      if (value) {
        weClient = value;
      }
    });

    const dispatch = createEventDispatcher();
    
    // export let coordinationHash: ActionHash;
    export let cHashData: any;
    let coordinationHash = cHashData ? decodeHashFromBase64(cHashData.coordinationHash) : undefined;
    export let filterType: string;
    
    let cHashAndClients;
    myCoordinations.subscribe(value => {
      if (value?.length > 0) {
        cHashAndClients = value;
      }
    })
    allCoordinations.subscribe(value => {
      if (value?.length > 0) {
        cHashAndClients = value;
      }
    })

    let hashB64 = encodeHashToBase64(coordinationHash)
    $: client = cHashAndClients.find(chc => chc.coordinationHash == hashB64)?.client;
    let clientBackup: AppClient = (getContext(clientContext) as any).getClient();
    
    let loading = true;
    let error: any = undefined;
    
    let record;//: Record | undefined;
    let coordRoles; //: Coordrole[] | undefined;
    let totalMin = 0;
    let totalUnderMin = 0;
    let totalParticipants = 0;
    let stringStartDate;
    let stringEndDate
    let stringExpiresDate
    let coordination_type;
    let firstGroupInfo;
    $: firstGroupInfo;
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

    let coordination: Coordination | undefined;
    allCoordinationsDetails.subscribe(value => {
      coordination = value[cHashData.coordinationHash];

      if (coordination) {
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
        totalParticipants = coordination.totalParticipants;
        totalMin = coordination.totalMin;
        totalUnderMin = coordination.totalUnderMin;
        loading = false;
      }
    })

    let errorSnackbar: Snackbar;
      
    $: error, loading, record, coordination;
    
    onMount(async () => {
      if (!client) {
        client = clientBackup;
      }
      let appletHash = appletHashFromAppId(client.installedAppId);
      let res = await getAppletInfoAndGroupsProfiles(weClient, appletHash);
      firstGroupInfo = Array.from(Object.values(res.groupProfiles)[0].entries())[0][1]
      refetchCoordinationDetails(client, coordinationHash);
      // fetchCoordination();
      // fetchRoles();
    });
    
    async function goToFullview() {
      navigate("coordination", coordinationHash);
    }

    // async function fetchCoordination() {
    //   loading = true;
    //   error = undefined;
    //   record = undefined;
    //   coordination = undefined;
      
    //   try {
    //     record = await client.callZome({
    //       cap_secret: null,
    //       role_name: 'whosin',
    //       zome_name: 'coordinator',
    //       fn_name: 'get_coordination',
    //       payload: coordinationHash,
    //     });
    //     if (record) {
    //       let options: Intl.DateTimeFormatOptions = { 
    //         weekday: 'long',
    //         year: 'numeric', 
    //         month: 'long',
    //         day: 'numeric', 
    //         hour: 'numeric', 
    //         minute: 'numeric', 
    //         hour12: true 
    //       };
    //       coordination = decode((record.entry as any).Present.entry) as Coordination;
    //       coordination_type = coordination.coordination_type
    //       stringStartDate = new Date(coordination.starts_date / 1000).toLocaleDateString(undefined, options);
    //       // if year is current year, don't show year
    //       stringStartDate = stringStartDate.replace(" " + new Date().getFullYear() + " ", " ");
    //       stringEndDate = new Date(coordination.ends_date / 1000).toLocaleDateString(undefined, options).replace(" " + new Date().getFullYear() + " ", " ");
    //       stringExpiresDate = new Date(coordination.signup_deadline / 1000).toLocaleDateString(undefined, options).replace(" " + new Date().getFullYear() + " ", " ");
    //     }
    //   } catch (e) {
    //     error = e;
    //   }
    
    //   loading = false;
    // }


    // async function fetchRoles() {
    //   loading = true;
    //   error = undefined;
    //   record = undefined;
    //   coordRoles = undefined;
      
    //   try {
    //     record = await client.callZome({
    //       cap_secret: null,
    //       role_name: 'whosin',
    //       zome_name: 'coordinator',
    //       fn_name: 'get_coordroles_for_coordination',
    //       payload: coordinationHash,
    //     });
    //     if (record) {
    //       record.forEach(r => {
    //         let min = decode(r.coordrole.entry.Present.entry)["minimum"];
    //         let underMin = Math.min(r.participants, min);
    //         totalParticipants += r.participants;
    //         totalMin += min;
    //         totalUnderMin += underMin;
    //         totalMin = totalMin;
    //         totalUnderMin = totalUnderMin;
    //       })
    //     } else {
    //       console.log("?")
    //     }
    //   } catch (e) {
    //     error = e;
    //   }

    //   loading = false;
    // }
    </script>
    
    <mwc-snackbar bind:this={errorSnackbar} leading>
    </mwc-snackbar>
    
    {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    {:else if error}
    <span>Error fetching the coordination: {error}</span>
    {:else if coordination?.title && coordination_type && coordination_type == filterType || filterType == "All"}
    <div on:mousedown={goToFullview} class="dashboard-item" style="margin-bottom: 8px;">
      <div style="display: flex; flex-direction: row; margin-bottom: 2px">
        <div class="action-title" style="display: flex; justify-content: space-between; width: 100%;"> 
          
          <div style="display: flex;">
            <div style="margin: auto; margin-right: 8px;">
              <!-- firstGropuInfo.icon_src render -->
              {#if weClient?.renderInfo.applets && firstGroupInfo}
              <img src={ firstGroupInfo ? firstGroupInfo.icon_src : "" } title={firstGroupInfo.name} style="width: 20px; height: 20px; border-radius: 50%; margin-bottom: -4px;"/>
              {/if}
              { coordination?.title }
            </div>
          </div>

          <div class="status-label" style="display: flex; font-size: 14px">
            <div class="type-label" style="background: {coordination_type_colors[coordination_type]}; margin: 7px; padding: 2px 10px; font-weight: 100; font-size: 14px">
              <SvgIcon style="height: 0" color="#fff" size=15 icon="{coordination_type_icons[coordination_type]}" /> 
              <div style="margin: 2px; height: 0;">
                {coordination_type}
              </div>
            </div>
            <!-- active, happening today, expired, gathering participation -->
            {#if coordination}
              {@const label = getCoordinationLabel(coordination)}
              <div style="background: {label.color}; color: #fff; padding: 3px 5px 0; border-radius: 5px; margin-right: 10px; margin: 7px;">
                {label.title}
              </div>
            {/if}
          </div>
        </div>
      </div>

      {#if coordination?.signup_deadline}
      <div class="action-details">
        <!-- deadline to signup -->
        <div class="action-date">
          <SvgIcon color="#484848" size=16 icon="faClock" />
          Deadline to signup: <span style="white-space: pre-line">{ stringExpiresDate }</span>
        </div>
      </div>
    {/if}

    {#if coordination?.starts_date}
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
    {:else if coordination?.ends_date}
      <div class="action-date">
        <SvgIcon color="#484848" size=16 icon="faClock" />
        Deadline to complete: <span style="white-space: pre-line">{ stringEndDate }</span>
      </div>
    {/if}

      <div style="display: flex; flex-direction: row; margin-bottom: 2px">
        <div class="action-description"> { coordination?.description }</div>
      </div>

      <div class="progress-extraouter" style="display: flex;">
        <div class="participation-meter participation-meter-outer">
          <div class="participation-progress" style="
          width: {totalMin > 0 ? totalUnderMin/totalMin * 100 : 100}%;
          background-color: rgba(255, 196, 0, 0.75);">
            &nbsp;
          </div>
        </div>
        <!-- <div class="participation-label"> -->
        <div class="participation-label" style="margin-top: 8px; margin-left: 8px; display: flex; flex-direction: row;">
          {#if totalUnderMin < totalMin}
            <span
              style="display: flex; width: max-content;"
            >
              {Math.round((totalUnderMin/totalMin) * 100)}%
            </span>
          {:else}
            ✔
          {/if}
        </div>
      </div>

      <!-- <div class="action-section">
        <div class="role-item">{totalParticipants} committed
          {#if totalUnderMin < totalMin}
            and {totalMin - totalUnderMin} more needed
          {:else}
            ✔
          {/if}
        </div>
      </div> -->

      <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px">
        <span style="white-space: pre-line">{ new Date(coordination.happening_date / 1000).toLocaleString() }</span>
      </div>    -->
    </div>
    {/if}
    
    