<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Coordrole } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
import '@material/mwc-slider';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let title: string = '';
let description: string = '';
let minimum: number = 0;
let maximum: number = 0;

let errorSnackbar: Snackbar;

$: title, description, minimum, maximum;
$: isCoordroleValid = true && title !== '' && description !== '' && true && true;

onMount(() => {
});

async function createCoordrole() {  
  const coordroleEntry: Coordrole = { 
    title: title!,
    description: description!,
    minimum: minimum!,
    maximum: maximum!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'create_coordrole',
      payload: coordroleEntry,
    });
    dispatch('coordrole-created', { coordroleHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the coordrole: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Coordrole</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Title" value={ title } on:input={e => { title = e.target.value; } } required></mwc-textfield>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Description" value={ description } on:input={e => { description = e.target.value; } } required></mwc-textfield>          
  </div>
            
  <div style="margin-bottom: 16px">
    <div style="display: flex; flex-direction: row">
      <span style="margin-right: 4px">Minimum</span>
    
      <mwc-slider value={ minimum } on:input={e => { minimum = e.detail.value; } } discrete></mwc-slider>
    </div>          
  </div>
            
  <div style="margin-bottom: 16px">
    <div style="display: flex; flex-direction: row">
      <span style="margin-right: 4px">Maximum</span>
    
      <mwc-slider value={ maximum } on:input={e => { maximum = e.detail.value; } } discrete></mwc-slider>
    </div>          
  </div>
            

  <mwc-button 
    raised
    label="Create Coordrole"
    disabled={!isCoordroleValid}
    on:click={() => createCoordrole()}
  ></mwc-button>
</div>
