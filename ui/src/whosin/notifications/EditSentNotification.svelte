<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { SentNotification } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalSentNotificationHash!: ActionHash;

export let currentRecord!: Record;
let currentSentNotification: SentNotification = decode((currentRecord.entry as any).Present.entry) as SentNotification;


let errorSnackbar: Snackbar;

$: ;
$: isSentNotificationValid = true;

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditSentNotification element`);
  }
  if (originalSentNotificationHash === undefined) {
    throw new Error(`The originalSentNotificationHash input is required for the EditSentNotification element`);
  }
});

async function updateSentNotification() {

  const sentNotification: SentNotification = { 
    unique_data: currentSentNotification.unique_data,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'update_sent_notification',
      payload: {
        original_sent_notification_hash: originalSentNotificationHash,
        previous_sent_notification_hash: currentRecord.signed_action.hashed.hash,
        updated_sent_notification: sentNotification
      }
    });
  
    dispatch('sent-notification-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the sent notification: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit SentNotification</span>
  

  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button 
      raised
      label="Save"
      disabled={!isSentNotificationValid}
      on:click={() => updateSentNotification()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
