<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { Contact } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalContactHash!: ActionHash;

export let currentRecord!: Record;
let currentContact: Contact = decode((currentRecord.entry as any).Present.entry) as Contact;

let textNumber: string | undefined = currentContact.text_number;
let whatsappNumber: string | undefined = currentContact.whatsapp_number;
let emailAddress: string | undefined = currentContact.email_address;

let errorSnackbar: Snackbar;

$: textNumber, whatsappNumber, emailAddress;
$: isContactValid = true;

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditContact element`);
  }
  if (originalContactHash === undefined) {
    throw new Error(`The originalContactHash input is required for the EditContact element`);
  }
});

async function updateContact() {

  const contact: Contact = { 
    text_number: textNumber,
    whatsapp_number: whatsappNumber,
    email_address: emailAddress,
    agent_pub_key: currentContact.agent_pub_key,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'update_contact',
      payload: {
        original_contact_hash: originalContactHash,
        previous_contact_hash: currentRecord.signed_action.hashed.hash,
        updated_contact: contact
      }
    });
  
    dispatch('contact-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the contact: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Contact</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Text Number" value={ textNumber } on:input={e => { textNumber = e.target.value;} } ></mwc-textarea>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Whatsapp Number" value={ whatsappNumber } on:input={e => { whatsappNumber = e.target.value;} } ></mwc-textarea>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Email Address" value={ emailAddress } on:input={e => { emailAddress = e.target.value;} } ></mwc-textarea>    
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
      disabled={!isContactValid}
      on:click={() => updateContact()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
