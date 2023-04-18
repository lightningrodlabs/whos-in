<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Contact } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

// export let notifier!: AgentPubKey;

let agentPubKey: AgentPubKey = client.myPubKey;
let textNumber: string | undefined = '';
let whatsappNumber: string | undefined = '';
let emailAddress: string | undefined = '';

let notifier = undefined; //: AgentPubKey | undefined;
let errorSnackbar: Snackbar;

$: agentPubKey, textNumber, whatsappNumber, emailAddress, notifier;
$: isContactValid = true;

onMount(() => {
  if (agentPubKey === undefined) {
    throw new Error(`The agentPubKey input is required for the CreateContact element`);
  }
});

async function createContact() {  
  const contactEntry: Contact = { 
    agent_pub_key: agentPubKey!,
    text_number: textNumber,
    whatsapp_number: whatsappNumber,
    email_address: emailAddress,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'send_contact',
      payload: contactEntry,
    });
    dispatch('contact-created', { contactHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the contact: ${e.data.data}`;
    errorSnackbar.show();
  }
}

async function findANotifier() {
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'find_a_notifier',
      payload: null,
    });
    notifier = record;
    dispatch('contact-created', { contactHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the contact: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>

<div class="white-container" style="display: flex; flex-direction: column">
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <h1 style="font-size: 24px; font-weight: 400; text-align: left;">Add your contact information</h1>

  
  <div style="margin-bottom: 16px">
    <mwc-textarea label="Text Number" value={ textNumber } on:input={e => { textNumber = e.target.value;} } ></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea label="Whatsapp Number" value={ whatsappNumber } on:input={e => { whatsappNumber = e.target.value;} } ></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea label="Email Address" value={ emailAddress } on:input={e => { emailAddress = e.target.value;} } ></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create Contact"
    disabled={!isContactValid}
    on:click={() => createContact()}
  ></mwc-button>
</div>
</div>