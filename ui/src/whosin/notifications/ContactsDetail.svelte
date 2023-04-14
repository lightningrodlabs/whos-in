<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Contacts } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditContacts from './EditContacts.svelte'; 

const dispatch = createEventDispatcher();

export let contactsHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let contacts: Contacts | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, contacts;

onMount(async () => {
  if (contactsHash === undefined) {
    throw new Error(`The contactsHash input is required for the ContactsDetail element`);
  }
  await fetchContacts();
});

async function fetchContacts() {
  loading = true;
  error = undefined;
  record = undefined;
  contacts = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'get_contacts',
      payload: contactsHash,
    });
    if (record) {
      contacts = decode((record.entry as any).Present.entry) as Contacts;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteContacts() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'notifications',
      fn_name: 'delete_contacts',
      payload: contactsHash,
    });
    dispatch('contacts-deleted', { contactsHash: contactsHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the contacts: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the contacts: {error.data.data}</span>
{:else if editing}
<EditContacts
  originalContactsHash={ contactsHash}
  currentRecord={record}
  on:contacts-updated={async () => {
    editing = false;
    await fetchContacts()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditContacts>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteContacts()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Text Number:</strong></span>
    <span style="white-space: pre-line">{ contacts.text_number }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Whatsapp Number:</strong></span>
    <span style="white-space: pre-line">{ contacts.whatsapp_number }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Email Address:</strong></span>
    <span style="white-space: pre-line">{ contacts.email_address }</span>
  </div>

</div>
{/if}

