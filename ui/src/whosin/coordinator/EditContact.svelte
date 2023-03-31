<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { Contact } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalContactHash!: ActionHash;

export let currentRecord!: Record;
let currentContact: Contact = decode((currentRecord.entry as any).Present.entry) as Contact;


let errorSnackbar: Snackbar;

$: ;
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
    agent_pub_key: currentContact.agent_pub_key,
    text_number: currentContact.text_number,
    whatsapp_number: currentContact.whatsapp_number,
    email: currentContact.email,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
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
