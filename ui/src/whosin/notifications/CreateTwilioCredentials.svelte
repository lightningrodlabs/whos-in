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
let fromNumberText: string = '';
let fromNumberWhatsapp: string = '';

let errorSnackbar: Snackbar;

$: accountSid, authToken, fromNumberText, fromNumberWhatsapp;
$: isTwilioCredentialsValid = true && accountSid !== '' && authToken !== '' && fromNumberText !== '' && fromNumberWhatsapp !== '';

onMount(() => {
});

async function createTwilioCredentials() {  
  const twilioCredentialsEntry: TwilioCredentials = { 
    account_sid: accountSid!,
    auth_token: authToken!,
    from_number_text: fromNumberText!,
    from_number_whatsapp: fromNumberWhatsapp!,
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
    console.log(e.data.data)
  }

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'claim_notifier',
      payload: "I am a great notifier.",
    });
  } catch (e) {
    console.log(e)
  } 
}

</script>
<div class="white-container" style="display: flex; flex-direction: column">
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <h1 style="font-size: 24px; font-weight: 400; text-align: left;">Add your Twilio credentials</h1>
  
  <div style="margin-bottom: 16px">
    <mwc-textarea label="Account Sid" value={ accountSid } on:input={e => { accountSid = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea label="Auth Token" value={ authToken } on:input={e => { authToken = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea label="From Number" value={ fromNumberText } on:input={e => { fromNumberText = e.target.value;} } required></mwc-textarea>          
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea label="From Number" value={ fromNumberWhatsapp } on:input={e => { fromNumberWhatsapp = e.target.value;} } required></mwc-textarea>          
  </div>

  <mwc-button 
    raised
    label="Save TwilioCredentials"
    disabled={!isTwilioCredentialsValid}
    on:click={() => createTwilioCredentials()}
  ></mwc-button>
</div>
</div>