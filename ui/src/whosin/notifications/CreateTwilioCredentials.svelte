<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { TwilioCredentials } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let accountSid: string = '';
let authToken: string = '';
let fromNumber: string = '';

let errorSnackbar: Snackbar;

$: accountSid, authToken, fromNumber;
$: isTwilioCredentialsValid = true && accountSid !== '' && authToken !== '' && fromNumber !== '';

onMount(() => {
});

async function createTwilioCredentials() {  
  const twilioCredentialsEntry: TwilioCredentials = { 
    account_sid: accountSid!,
    auth_token: authToken!,
    from_number: fromNumber!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'create_twilio_credentials',
      payload: twilioCredentialsEntry,
    });
    dispatch('twilio-credentials-created', { twilioCredentialsHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the twilio credentials: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create TwilioCredentials</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Account Sid" value={ accountSid } on:input={e => { accountSid = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Auth Token" value={ authToken } on:input={e => { authToken = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="From Number" value={ fromNumber } on:input={e => { fromNumber = e.target.value;} } required></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create TwilioCredentials"
    disabled={!isTwilioCredentialsValid}
    on:click={() => createTwilioCredentials()}
  ></mwc-button>
</div>
