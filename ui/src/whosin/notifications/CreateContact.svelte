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
$: isContactValid = textNumber !== '' || whatsappNumber !== '' || emailAddress !== '';

onMount(() => {
  console.log('' + agentPubKey + '')

  if (agentPubKey === undefined) {
    throw new Error(`The agentPubKey input is required for the CreateContact element`);
  }
});

async function createContact() {  
  console.log(agentPubKey)
  let fakePubKey = new Uint8Array([132,32,36,103,50,63,238,220,212,245,234,17,72,63,223,125,210,149,78,140,62,68,71,132,168,197,88,98,96,105,222,102,49,182,16,86,126,96,12])
  const contactEntry: Contact = {
    agent_pub_key: agentPubKey!,
    // agent_pub_key: fakePubKey!,
    text_number: textNumber,
    whatsapp_number: whatsappNumber,
    email_address: emailAddress,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'select_first_notifier',
      payload: null,
    });
  } catch (e) {
    console.log(e)
    errorSnackbar.labelText = `Error creating the contact: ${e.data.data}`;
    errorSnackbar.show();
  }

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'send_contact',
      payload: contactEntry,
    });
    dispatch('contact-created', { contactHash: record });
  } catch (e) {
    console.log(e)
    errorSnackbar.labelText = `Error creating the contact: ${e}`;
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
    label="Save Contact Information"
    disabled={!isContactValid}
    on:click={() => createContact()}
  ></mwc-button>
</div>
</div>