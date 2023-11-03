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

let loadingNotifiers = true;

let agentPubKey: AgentPubKey = client.myPubKey;
let textNumber: string | undefined = '';
let whatsappNumber: string | undefined = '';
let emailAddress: string | undefined = '';

let notifier: AgentPubKey | undefined;
let errorSnackbar: Snackbar;
let allNotifiers: AgentPubKey[] = [];

$: agentPubKey, textNumber, whatsappNumber, emailAddress, notifier, allNotifiers, loadingNotifiers;
$: isContactValid = textNumber !== '' || whatsappNumber !== '' || emailAddress !== '';

async function getNotifiers() {
  loadingNotifiers = true;
  try {
    const records: AgentPubKey[] = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'list_notifiers',
      payload: null,
    });
    console.log(records)
    // only one of each record.join(",")
    // allNotifiers = records.filter((item, index) => records[index].join("") === item.join(""));
    records.forEach((item, index) => {
      if (!allNotifiers?.includes(item.agent.join(","))) {
        allNotifiers?.push(item);
      }
    });
    setInterval(() => {
      loadingNotifiers = false;
    }, 2000);
  } catch (e) {
    console.log(e)
    errorSnackbar.labelText = `Error creating the contact: ${e.data.data}`;
    errorSnackbar.show();
  }
}

onMount(async () => {
  await getNotifiers();

  // console.log('' + agentPubKey + '')

  if (agentPubKey === undefined) {
    throw new Error(`The agentPubKey input is required for the CreateContact element`);
  }
});

async function createContact() {  
  console.log(notifier)
  // console.log(agentPubKey)
  // let fakePubKey = new Uint8Array([132,32,36,103,50,63,238,220,212,245,234,17,72,63,223,125,210,149,78,140,62,68,71,132,168,197,88,98,96,105,222,102,49,182,16,86,126,96,12])
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
      fn_name: 'select_notifier',
      payload: notifier,
    });
  } catch (e) {
    console.log(e)
    console.log(e.data.data)
    errorSnackbar.labelText = `Error selecting notifier: ${e.data.data}`;
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

// async function findANotifier() {
//   try {
//     const record: Record = await client.callZome({
//       cap_secret: null,
//       role_name: 'whosin',
//       zome_name: 'notifications',
//       fn_name: 'find_a_notifier',
//       payload: null,
//     });
//     notifier = record;
//     dispatch('contact-created', { contactHash: record.signed_action.hashed.hash });
//   } catch (e) {
//     errorSnackbar.labelText = `Error creating the contact: ${e.data.data}`;
//     errorSnackbar.show();
//   }
// }

let regionCode = "";
let phoneNumber = "";
let whatsappRegionCode = "";
let whatsappPhoneNumber = "";

const handleRegionInput = (event) => {
  let input = event.target.value.replace(/[^+\d]/g, '');
  if (!input.startsWith('+')) input = '+' + input;
  regionCode = input;
  textNumber = regionCode + phoneNumber;
};

const handlePhoneInput = (event) => {
  let input = event.target.value.replace(/\D/g, '');
  if (input.length > 3 && input.length <= 6)
      input = `(${input.slice(0, 3)}) ${input.slice(3)}`;
  else if (input.length > 6)
      input = `(${input.slice(0, 3)}) ${input.slice(3, 6)}-${input.slice(6, 10)}`;
  phoneNumber = input;
  textNumber = regionCode + phoneNumber;
};

const handleWhatsappRegionInput = (event) => {
  let input = event.target.value.replace(/[^+\d]/g, '');
  if (!input.startsWith('+')) input = '+' + input;
  whatsappRegionCode = input;
  whatsappNumber = whatsappRegionCode + whatsappPhoneNumber;
};

const handleWhatsappPhoneInput = (event) => {
  let input = event.target.value.replace(/\D/g, '');
  if (input.length > 3 && input.length <= 6)
      input = `(${input.slice(0, 3)}) ${input.slice(3)}`;
  else if (input.length > 6)
      input = `(${input.slice(0, 3)}) ${input.slice(3, 6)}-${input.slice(6, 10)}`;
  whatsappPhoneNumber = input;
  whatsappNumber = whatsappRegionCode + whatsappPhoneNumber;
};

</script>

<div class="white-container" style="display: flex; flex-direction: column">
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <h1 style="font-size: 24px; font-weight: 400; text-align: left;">Add your contact information</h1>

  
  <!-- <div style="margin-bottom: 16px">
    <mwc-textarea label="Text Number" value={ textNumber } on:input={e => { textNumber = e.target.value;} } ></mwc-textarea>     
  </div> -->
  
  <!-- <div class="form-container">
    <mwc-textfield label="Phone number" type="tel" inputmode="numeric" value={ textNumber }  on:input={handleInput}></mwc-textfield>
  </div> -->
{#if loadingNotifiers}
  <div>Loading notifiers...</div>
{:else if allNotifiers && allNotifiers.length}
  <div>
    <select bind:value={notifier}>
      {#each allNotifiers as n}
        <option value={n.agent}>{n.tag}</option>
      {/each}
  </div>
  <br>
  
  <div style="margin-bottom: 16px">
    <div style="margin-bottom: 16px; display: block"><label>Text Number</label></div>
    <mwc-textfield style="width:80px; display:inline-block" label="region" type="tel" inputmode="numeric" value={regionCode} on:input={handleRegionInput}></mwc-textfield>
    <mwc-textfield style="width:240px; display:inline-block" label="number" type="tel" inputmode="numeric" value={phoneNumber} on:input={handlePhoneInput}></mwc-textfield>
  </div>

  <div style="margin-bottom: 16px">
    <div style="margin-bottom: 16px; display: block"><label>Whatsapp Number</label></div>
    <mwc-textfield style="width:80px; display:inline-block" label="region" type="tel" inputmode="numeric" value={whatsappRegionCode} on:input={handleWhatsappRegionInput}></mwc-textfield>
    <mwc-textfield style="width:240px; display:inline-block" label="number" type="tel" inputmode="numeric" value={whatsappPhoneNumber} on:input={handleWhatsappPhoneInput}></mwc-textfield>
  </div>

  <!-- <div style="margin-bottom: 16px">
    <mwc-textarea label="Whatsapp Number" value={ whatsappNumber } on:input={e => { whatsappNumber = e.target.value;} } ></mwc-textarea>          
  </div> -->
            
  <div style="margin-bottom: 16px; display: block"><label>Email Address</label></div>
  <div style="margin-bottom: 16px">
    <mwc-textarea label="*******@****.***" value={ emailAddress } on:input={e => { emailAddress = e.target.value;} } ></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Save Contact Information"
    disabled={!isContactValid}
    on:click={() => createContact()}
  ></mwc-button>

{:else}
  <div>Couldn't find any notifiers.
  <button on:click={() => getNotifiers()}>Try again</button></div>
{/if}
</div>
</div>
