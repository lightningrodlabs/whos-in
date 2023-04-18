<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { SentNotification } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let uniqueData!: string;



let errorSnackbar: Snackbar;

$: uniqueData;
$: isSentNotificationValid = true;

onMount(() => {
  if (uniqueData === undefined) {
    throw new Error(`The uniqueData input is required for the CreateSentNotification element`);
  }
});

async function createSentNotification() {  
  const sentNotificationEntry: SentNotification = { 
    unique_data: uniqueData!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'create_sent_notification',
      payload: sentNotificationEntry,
    });
    dispatch('sent-notification-created', { sentNotificationHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the sent notification: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create SentNotification</span>
  


  <mwc-button 
    raised
    label="Create SentNotification"
    disabled={!isSentNotificationValid}
    on:click={() => createSentNotification()}
  ></mwc-button>
</div>
