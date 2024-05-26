<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import { decode } from '@msgpack/msgpack';
  import type { Record, ActionHash, AppClient, EntryHash, AgentPubKey } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Coordrole } from './types';
  import '@material/mwc-circular-progress';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-snackbar';
  import '@material/mwc-icon-button';
  
  const dispatch = createEventDispatcher();
  
  export let coordroleHash: ActionHash;
  
  let client: AppClient = (getContext(clientContext) as any).getClient();
  
  let loading = true;
  let error: any = undefined;
  
  let record: Record | undefined;
  let coordrole: Coordrole | undefined;
  
  let errorSnackbar: Snackbar;
    
  $: error, loading, record, coordrole;
  
  onMount(() => fetchCoordrole());
  
  async function fetchCoordrole() {
    loading = true;
    error = undefined;
    record = undefined;
    coordrole = undefined;
    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'get_coordrole',
        payload: coordroleHash,
      });
      if (record) {
        coordrole = decode((record.entry as any).Present.entry) as Coordrole;
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
  <span>Error fetching the coordrole: {error.data.data}</span>
  {:else}
  
  <div style="display: flex; flex-direction: column">
    <div style="display: flex; flex-direction: row">
      <span style="font-size: 18px; flex: 1;">Coordrole</span>
    </div>
  
    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span><strong>Title</strong></span>
      <span style="white-space: pre-line">{ coordrole.title }</span>
    </div>
  
    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span><strong>Description</strong></span>
      <span style="white-space: pre-line">{ coordrole.description }</span>
    </div>
  
  </div>
  {/if}
  
  