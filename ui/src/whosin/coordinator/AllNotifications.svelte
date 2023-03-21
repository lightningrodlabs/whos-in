<script lang="ts">
    import { onMount, getContext } from 'svelte';
    import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
    import { clientContext } from '../../contexts';
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

    async function goToCoordination(coordinationHash, seen) {

    if (!seen) {
        try {
            const records = await client
            .callZome({
                cap_secret: null,
                role_name: 'whosin',
                zome_name: 'coordinator',
                fn_name: 'add_coordination_for_viewer',
                payload: coordinationHash,
            });
            navigate("coordination", coordinationHash);
            records
        } catch (e) {
            error = e;
        }
    } else {
        navigate("coordination", coordinationHash);
    }
}

</script>

<!-- { JSON.stringify(error) } -->
<div class="invisible-outer">
    <h1>Notifications</h1>
    <ul id="notifications">
    {#each local_notifications as n}
        {#if !n.seen}
            <li on:click={goToCoordination(n.hash, n.seen)}><b>{n.description}</b></li>
        {:else}
            <li on:click={goToCoordination(n.hash, n.seen)}>{n.description}</li>
        {/if}
    {/each}
    </ul>
</div>
