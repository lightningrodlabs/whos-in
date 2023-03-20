<script lang="ts">
    import { onMount, getContext } from 'svelte';
    import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
    import { clientContext } from '../../contexts';
    // import type { SeenNotifications, Seen } from './types';
    import { decode } from '@msgpack/msgpack';
    import { notifications, notifications_update } from '../../store.js';
    import { navigate } from '../../store.js';
    
    let client: AppAgentClient = (getContext(clientContext) as any).getClient();
    
    let coordinations: Array<ActionHash> | undefined;
    let local_notifications;
    let loading = true;
    let error: any = undefined;
    
    $: coordinations, loading, error;
    
    onMount(async () => {
    });

    notifications.subscribe(value => {
        local_notifications = value;
    });


    // async function markSeen(content: String) {
    //     try {
    //         const records = await client
    //         .callZome({
    //             cap_secret: null,
    //             role_name: 'hcan',
    //             zome_name: 'coordinator',
    //             fn_name: 'register_seen',
    //             payload: content,
    //         });
    //         records
    //     } catch (e) {
    //         error = e;
    //     }
    // }

    // async function markSeen(coordinationHash: EntryHash, description: string) {  
    //     const seenEntry: Seen = { 
    //         coordination_hash: coordinationHash!,
    //         description: description!,
    //     };
        
    //     try {
    //         const record: Record = await client.callZome({
    //         cap_secret: null,
    //         role_name: 'hcan',
    //         zome_name: 'coordinator',
    //         fn_name: 'create_seen',
    //         payload: seenEntry,
    //         });
    //         // dispatch('seen-created', { seenHash: record.signed_action.hashed.hash });
    //     } catch (e) {
    //         alert(JSON.stringify(e))
    //         // errorSnackbar.labelText = `Error creating the seen: ${e.data.data}`;
    //         // errorSnackbar.show();
    //     }
    // }

    // async function checkSeen(coordinationHash: ActionHash, description: string) {
    //     const seenEntry: SeenNotifications = { 
    //         coordination_hash: coordinationHash!,
    //         description: description!,
    //     };

    //     try {
    //         const records = await client
    //         .callZome({
    //             cap_secret: null,
    //             role_name: 'hcan',
    //             zome_name: 'coordinator',
    //             fn_name: 'get_seen',
    //             payload: seenEntry,
    //         });
    //         // alert(records)
    //         records
    //     } catch (e) {
    //         error = e;
    //     }
    // }

    async function goToCoordination(coordinationHash, description) {
    //   markSeen(coordinationHash, description)
      navigate("coordination", coordinationHash);
    }

</script>

<!-- { JSON.stringify(error) } -->
<div class="invisible-outer">
    <h1>Notifications</h1>
    <ul id="notifications">
    {#each local_notifications as n}
        <!-- {#if checkSeen(n.hash, n.description)}
            {JSON.stringify(checkSeen(n.hash, n.description))}
        {/if} -->
        <!-- {JSON.stringify(n)} -->
        <li on:click={goToCoordination(n.hash, n.description)}>{n.description}</li>
    {/each}
    </ul>
</div>
