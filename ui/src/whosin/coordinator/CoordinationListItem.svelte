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
    import { view, viewHash, navigate } from '../../store.js';

    const dispatch = createEventDispatcher();
    
    export let coordinationHash: ActionHash;
    
    let client: AppAgentClient = (getContext(clientContext) as any).getClient();
    
    let loading = true;
    let error: any = undefined;
    
    let record: Record | undefined;
    let coordination: Coordination | undefined;
    let coordRoles; //: Coordrole[] | undefined;
    let totalMin = 0;
    let totalUnderMin = 0;
    let totalParticipants = 0;
    
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
          coordination = decode((record.entry as any).Present.entry) as Coordination;
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
    {:else}
    
    <div on:click={goToFullview} class="dashboard-item" style="margin-bottom: 8px;">
      <div style="display: flex; flex-direction: row; margin-bottom: 2px">
        <div class="action-title"> { coordination.title }</div>
      </div>

      <!-- <div class="progress-extraouter"> -->
        <div class="participation-meter participation-meter-outer">
          <div class="participation-progress" style="
          width: {totalUnderMin/totalMin * 100}%;
          background-color: rgba(255, 196, 0, 0.75);">
            &nbsp;
          </div>
        </div>
      <!-- </div> -->

      <div class="action-section">
        <div class="role-item">{totalParticipants} committed
          {#if totalUnderMin < totalMin}
            and {totalMin - totalUnderMin} more needed
          {:else}
            ✔
          {/if}
        </div>
      </div>

      <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px">
        <span style="white-space: pre-line">{ new Date(coordination.happening_date / 1000).toLocaleString() }</span>
      </div>    -->
    </div>
    {/if}
    
    