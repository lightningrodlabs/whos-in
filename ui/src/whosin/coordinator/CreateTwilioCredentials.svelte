<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { TwilioCredentials } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let accountSid: string = '';
let authToken: string = '';

let errorSnackbar: Snackbar;

$: accountSid, authToken;
$: isTwilioCredentialsValid = true && accountSid !== '' && authToken !== '';

onMount(() => {
});

async function createTwilioCredentials() {  
  const twilioCredentialsEntry: TwilioCredentials = { 
    account_sid: accountSid!,
    auth_token: authToken!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'create_twilio_credentials',
      payload: twilioCredentialsEntry,
    });
    dispatch('twilio-credentials-created', { twilioCredentialsHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the twilio credentials: ${e.data.data}`;
    errorSnackbar.show();
  }

  try {
    console.log('start')
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'grant_unrestricted_capability',
      payload: null,
    });
    console.log(record)
  } catch (e) {
    errorSnackbar.labelText = `Error creating the cap grant: ${e.data.data}`;
    errorSnackbar.show();
    console.log(e.data.data)
  }
  console.log('end')
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
            

  <mwc-button 
    raised
    label="Create TwilioCredentials"
    disabled={!isTwilioCredentialsValid}
    on:click={() => createTwilioCredentials()}
  ></mwc-button>
</div>
