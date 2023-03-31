<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Contact } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditContact from './EditContact.svelte'; 

const dispatch = createEventDispatcher();

export let contactHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let contact: Contact | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, contact;

onMount(async () => {
  if (contactHash === undefined) {
    throw new Error(`The contactHash input is required for the ContactDetail element`);
  }
  await fetchContact();
});

async function fetchContact() {
  loading = true;
  error = undefined;
  record = undefined;
  contact = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_contact',
      payload: contactHash,
    });
    if (record) {
      contact = decode((record.entry as any).Present.entry) as Contact;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteContact() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'delete_contact',
      payload: contactHash,
    });
    dispatch('contact-deleted', { contactHash: contactHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the contact: ${e.data.data}`;
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
<span>Error fetching the contact: {error.data.data}</span>
{:else if editing}
<EditContact
  originalContactHash={ contactHash}
  currentRecord={record}
  on:contact-updated={async () => {
    editing = false;
    await fetchContact()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditContact>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteContact()}></mwc-icon-button>
  </div>

</div>
{/if}

