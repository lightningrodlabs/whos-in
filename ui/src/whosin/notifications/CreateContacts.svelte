<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Contacts } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let agentPubKey!: AgentPubKey;


let textNumber: string | undefined = '';
let whatsappNumber: string | undefined = '';
let emailAddress: string | undefined = '';

let errorSnackbar: Snackbar;

$: agentPubKey, textNumber, whatsappNumber, emailAddress;
$: isContactsValid = true;

onMount(() => {
  if (agentPubKey === undefined) {
    throw new Error(`The agentPubKey input is required for the CreateContacts element`);
  }
});

async function createContacts() {  
  const contactsEntry: Contacts = { 
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
      fn_name: 'create_contacts',
      payload: contactsEntry,
    });
    dispatch('contacts-created', { contactsHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the contacts: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Contacts</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Text Number" value={ textNumber } on:input={e => { textNumber = e.target.value;} } ></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Whatsapp Number" value={ whatsappNumber } on:input={e => { whatsappNumber = e.target.value;} } ></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Email Address" value={ emailAddress } on:input={e => { emailAddress = e.target.value;} } ></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create Contacts"
    disabled={!isContactsValid}
    on:click={() => createContacts()}
  ></mwc-button>
</div>
