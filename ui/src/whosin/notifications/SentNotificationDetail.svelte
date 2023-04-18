<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { SentNotification } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditSentNotification from './EditSentNotification.svelte'; 

const dispatch = createEventDispatcher();

export let sentNotificationHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let sentNotification: SentNotification | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, sentNotification;

onMount(async () => {
  if (sentNotificationHash === undefined) {
    throw new Error(`The sentNotificationHash input is required for the SentNotificationDetail element`);
  }
  await fetchSentNotification();
});

async function fetchSentNotification() {
  loading = true;
  error = undefined;
  record = undefined;
  sentNotification = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'get_sent_notification',
      payload: sentNotificationHash,
    });
    if (record) {
      sentNotification = decode((record.entry as any).Present.entry) as SentNotification;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteSentNotification() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'delete_sent_notification',
      payload: sentNotificationHash,
    });
    dispatch('sent-notification-deleted', { sentNotificationHash: sentNotificationHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the sent notification: ${e.data.data}`;
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
<span>Error fetching the sent notification: {error.data.data}</span>
{:else if editing}
<EditSentNotification
  originalSentNotificationHash={ sentNotificationHash}
  currentRecord={record}
  on:sent-notification-updated={async () => {
    editing = false;
    await fetchSentNotification()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditSentNotification>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteSentNotification()}></mwc-icon-button>
  </div>

</div>
{/if}

