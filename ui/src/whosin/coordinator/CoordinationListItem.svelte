<script lang="ts">
    import { createEventDispatcher, onMount, getContext } from 'svelte';
    import '@material/mwc-circular-progress';
    import { decode } from '@msgpack/msgpack';
    import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey } from '@holochain/client';
    import { clientContext } from '../../contexts';
    import type { Coordination } from './types';
    import '@material/mwc-circular-progress';
    import type { Snackbar } from '@material/mwc-snackbar';
    import '@material/mwc-snackbar';
    import '@material/mwc-icon-button';
    import SvgIcon from './SvgIcon.svelte';
    import { view, viewHash, navigate } from '../../store.js';

    const dispatch = createEventDispatcher();
    
    export let coordinationHash: ActionHash;
    export let filterType: string;
    
    let client: AppAgentClient = (getContext(clientContext) as any).getClient();
    
    let loading = true;
    let error: any = undefined;
    
    let record;//: Record | undefined;
    let coordination: Coordination | undefined;
    let coordRoles; //: Coordrole[] | undefined;
    let totalMin = 0;
    let totalUnderMin = 0;
    let totalParticipants = 0;
    let stringStartDate;
    let stringEndDate
    let stringExpiresDate
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
      
    $: error, loading, record, coordination;
    
    onMount(() => {
      fetchCoordination();
      fetchRoles();
    });
    
    async function goToFullview() {
      navigate("coordination", coordinationHash);
    }

    async function fetchCoordination() {
      loading = true;
      error = undefined;
      record = undefined;
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
          let options: Intl.DateTimeFormatOptions = { 
            weekday: 'long',
            year: 'numeric', 
            month: 'long',
            day: 'numeric', 
            hour: 'numeric', 
            minute: 'numeric', 
            hour12: true 
          };
          coordination = decode((record.entry as any).Present.entry) as Coordination;
          coordination_type = Object.keys(coordination.coordination_type)[0]
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
      loading = true;
      error = undefined;
      record = undefined;
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
        } else {
          console.log("?")
        }
      } catch (e) {
        error = e;
      }

      loading = false;
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
    {:else if coordination_type == filterType || filterType == "All"}
    <div on:mousedown={goToFullview} class="dashboard-item" style="margin-bottom: 8px;">
      <div style="display: flex; flex-direction: row; margin-bottom: 2px">
        <div class="action-title" style="display: flex; justify-content: space-between; width: 100%;"> 
          
          <div style="display: flex;">
            <div style="margin: auto; margin-right: 8px;">
              { coordination.title }
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
            {#if coordination.ends_date && coordination.ends_date < (new Date().getTime() * 1000)}
              <div style="background: #ff0000; color: #fff; padding: 3px 5px; border-radius: 5px; margin-right: 10px; margin: 7px;">
                Expired
              </div>
            {:else if totalUnderMin >= totalMin && coordination.starts_date && coordination.starts_date < (new Date().getTime() * 1000) && ((coordination.starts_date < (new Date().getTime() * 1000 + (24 * 60 * 60 * 1000)))) || ((!coordination.ends_date || coordination.ends_date > (new Date().getTime() * 1000)))}
              <div style="background: #cd1dff; color: #fff; padding: 3px 5px; border-radius: 5px; margin-right: 10px; margin: 7px;">
                Happening today
              </div>
            {:else if totalMin > 0 && totalUnderMin < totalMin}
              <div style="background: #ff951d; color: #fff; padding: 3px 5px; border-radius: 5px; margin-right: 10px; margin: 7px;">
                Gathering participation
              </div>
            {:else if totalUnderMin >= totalMin}
              <div style="background: #57ca01; color: #fff; padding: 3px 5px; border-radius: 5px; margin-right: 10px; margin: 7px;">
                Active
              </div>
            {/if}
          </div>
        </div>
      </div>

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

      <div style="display: flex; flex-direction: row; margin-bottom: 2px">
        <div class="action-description"> { coordination.description }</div>
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
              {totalUnderMin/totalMin}%
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
    
    