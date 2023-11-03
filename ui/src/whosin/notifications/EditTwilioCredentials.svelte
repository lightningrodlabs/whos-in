<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { TwilioCredentials } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalTwilioCredentialsHash!: ActionHash;

export let currentRecord!: Record;
let currentTwilioCredentials: TwilioCredentials = decode((currentRecord.entry as any).Present.entry) as TwilioCredentials;

let accountSid: string | undefined = currentTwilioCredentials.account_sid;
let authToken: string | undefined = currentTwilioCredentials.auth_token;
let fromNumber: string | undefined = currentTwilioCredentials.from_number;

let errorSnackbar: Snackbar;

$: accountSid, authToken, fromNumber;
$: isTwilioCredentialsValid = true && accountSid !== '' && authToken !== '' && fromNumber !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditTwilioCredentials element`);
  }
  if (originalTwilioCredentialsHash === undefined) {
    throw new Error(`The originalTwilioCredentialsHash input is required for the EditTwilioCredentials element`);
  }
});

async function updateTwilioCredentials() {

  const twilioCredentials: TwilioCredentials = { 
    account_sid: accountSid!,
    auth_token: authToken!,
    from_number: fromNumber!,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'update_twilio_credentials',
      payload: {
        original_twilio_credentials_hash: originalTwilioCredentialsHash,
        previous_twilio_credentials_hash: currentRecord.signed_action.hashed.hash,
        updated_twilio_credentials: twilioCredentials
      }
    });
  
    dispatch('twilio-credentials-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the twilio credentials: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit TwilioCredentials</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Account Sid" value={ accountSid } on:input={e => { accountSid = e.target.value;} } required></mwc-textarea>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Auth Token" value={ authToken } on:input={e => { authToken = e.target.value;} } required></mwc-textarea>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="From Number" value={ fromNumber } on:input={e => { fromNumber = e.target.value;} } required></mwc-textarea>    
  </div>


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
      disabled={!isTwilioCredentialsValid}
      on:click={() => updateTwilioCredentials()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
