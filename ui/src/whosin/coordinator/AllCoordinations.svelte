<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { CoordinatorSignal } from './types';
import CoordinationListItem from './CoordinationListItem.svelte';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  await fetchCoordinations();
  client.on('signal', signal => {
    if (signal.zome_name !== 'coordinator') return;
    const payload = signal.payload as CoordinatorSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Coordination') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchCoordinations() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_all_coordinations',
      payload: null,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the coordinations: {error.data.data}.</span>
{:else if hashes.length === 0}
<div class="white-container" style="display: flex; flex-direction: column">
  <h1>Actions Bulletin</h1>
  <span>No actions yet.</span>
</div>
{:else}
<div class="white-container" style="display: flex; flex-direction: column">
  <h1>Actions Bulletin</h1>
  {#each hashes as hash}
    <CoordinationListItem coordinationHash={hash}></CoordinationListItem>
  {/each}
</div>
{/if}

