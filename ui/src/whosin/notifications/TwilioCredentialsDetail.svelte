<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { TwilioCredentials } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditTwilioCredentials from './EditTwilioCredentials.svelte'; 

const dispatch = createEventDispatcher();

// export let twilioCredentialsHash: ActionHash;

let client: AppClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let twilioCredentials: TwilioCredentials | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, twilioCredentials;

onMount(async () => {
  // if (twilioCredentialsHash === undefined) {
  //   throw new Error(`The twilioCredentialsHash input is required for the TwilioCredentialsDetail element`);
  // }
  // await fetchTwilioCredentials();
});

// async function fetchTwilioCredentials() {
//   loading = true;
//   error = undefined;
//   record = undefined;
//   twilioCredentials = undefined;
  
//   try {
//     record = await client.callZome({
//       cap_secret: null,
//       role_name: 'whosin',
//       zome_name: 'notifications',
//       fn_name: 'get_twilio_credentials',
//       payload: twilioCredentialsHash,
//     });
//     if (record) {
//       twilioCredentials = decode((record.entry as any).Present.entry) as TwilioCredentials;
//     }
//   } catch (e) {
//     error = e;
//   }

//   loading = false;
// }

// async function deleteTwilioCredentials() {
//   try {
//     await client.callZome({
//       cap_secret: null,
//       role_name: 'whosin',
//       zome_name: 'notifications',
//       fn_name: 'delete_twilio_credentials',
//       payload: twilioCredentialsHash,
//     });
//     dispatch('twilio-credentials-deleted', { twilioCredentialsHash: twilioCredentialsHash });
//   } catch (e: any) {
//     errorSnackbar.labelText = `Error deleting the twilio credentials: ${e.data.data}`;
//     errorSnackbar.show();
//   }
// }
</script>

<!-- <mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the twilio credentials: {error.data.data}</span>
{:else if editing}
<EditTwilioCredentials
  originalTwilioCredentialsHash={ twilioCredentialsHash}
  currentRecord={record}
  on:twilio-credentials-updated={async () => {
    editing = false;
    await fetchTwilioCredentials()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditTwilioCredentials>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteTwilioCredentials()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Account Sid:</strong></span>
    <span style="white-space: pre-line">{ twilioCredentials.account_sid }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Auth Token:</strong></span>
    <span style="white-space: pre-line">{ twilioCredentials.auth_token }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>From Number:</strong></span>
    <span style="white-space: pre-line">{ twilioCredentials.from_number }</span>
  </div>

</div>
{/if}
 -->
