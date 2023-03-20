<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import CoordinationDetail from './CoordinationDetail.svelte';
import type { CoordinatorSignal } from './types';

export let viewer: AgentPubKey;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (viewer === undefined) {
    throw new Error(`The viewer input is required for the CoordinationsForViewer element`);
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_coordinations_for_viewer',
      payload: viewer,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;

  client.on('signal', signal => {
    if (signal.zome_name !== 'coordinator') return;
    const payload = signal.payload as CoordinatorSignal;
    if (payload.type !== 'LinkCreated') return;
    if (payload.link_type !== 'ViewerToCoordinations') return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching coordinations: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No coordinations found for this viewer.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <CoordinationDetail coordinationHash={hash}></CoordinationDetail>
    </div>
  {/each}
</div>
{/if}
