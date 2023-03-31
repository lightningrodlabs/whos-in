<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Contact } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

// my pubkey
let agentPubKey: AgentPubKey = client.myPubKey;

// export let textNumber!: string;

// export let whatsappNumber!: string;

// export let email!: string;



let errorSnackbar: Snackbar;

$: agentPubKey//, textNumber, whatsappNumber, email;
$: isContactValid = true;

onMount(() => {
  if (agentPubKey === undefined) {
    throw new Error(`The agentPubKey input is required for the CreateContact element`);
  }
  // if (textNumber === undefined) {
  //   throw new Error(`The textNumber input is required for the CreateContact element`);
  // }
  // if (whatsappNumber === undefined) {
  //   throw new Error(`The whatsappNumber input is required for the CreateContact element`);
  // }
  // if (email === undefined) {
  //   throw new Error(`The email input is required for the CreateContact element`);
  // }
});

async function createContact() {  
  const contactEntry: Contact = { 
    agent_pub_key: agentPubKey!,
    text_number: "textNumber!",
    whatsapp_number: "whatsappNumber!",
    email: "email!",
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'create_contact',
      payload: contactEntry,
    });
    dispatch('contact-created', { contactHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the contact: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Contact</span>
  


  <mwc-button 
    raised
    label="Create Contact"
    disabled={!isContactValid}
    on:click={() => createContact()}
  ></mwc-button>
</div>
