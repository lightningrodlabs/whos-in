<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { type EntryHash, type Record, type AgentPubKey, type ActionHash, type AppAgentClient, type NewEntryAction, type decodeHashFromBase64, encodeHashToBase64 } from '@holochain/client';
import { clientContext } from '../../contexts';
import { viewed, setViewed, addToViewed } from '../../store.js';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  await fetchVieweds();
});

async function fetchVieweds() {
  console.log('1')
  try {
    console.log('2')
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_all_viewed',
      payload: null,
    });
    console.log('3')
    console.log("all viewed records: ", records)
    setViewed(records.map(r => 
      encodeHashToBase64(r.viewed_hash)
    ));
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>
