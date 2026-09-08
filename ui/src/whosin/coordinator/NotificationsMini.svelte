<script lang="ts">
    import { onMount, getContext } from 'svelte';
    import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient } from '@holochain/client';
    import { clientContext } from '../../contexts';
    import { notifications, navigate, setSeenNotification } from '../../store.js';
    
    // Declared as a prop with the context read as its default, so both call
    // styles work: Header.svelte passes client={client}, while the other 29
    // components rely on context alone. Previously the prop declaration was
    // commented out here, so the passed prop was ignored and Svelte warned
    // "was created with unknown prop 'client'".
    //
    // The context read is a snapshot taken at component-init time, not a live
    // binding. It only yields a real client because App.svelte gates this whole
    // subtree behind {#if client || applets != undefined}; a child created
    // outside that gate would snapshot undefined and stay that way.
    export let client: AppClient = (getContext(clientContext) as any).getClient();
    
    let coordinations: Array<ActionHash> | undefined;
    let local_notifications;
    let loading = true;
    let error: any = undefined;
    let notifier = undefined;
    
    $: coordinations, loading, error, notifier;

    notifications.subscribe(value => {
        local_notifications = value;
    });

    async function seeNotification(notification) {
        setSeenNotification(notification)
        try {
            await client
            .callZome({
                cap_secret: null,
                role_name: 'whosin',
                zome_name: 'coordinator',
                fn_name: 'add_coordination_for_viewer',
                payload: notification.hash,
            });
        } catch (e) {
            error = e;
        }
    }

    async function goToCoordination(notification) {
        if (!notification.seen) {
            await seeNotification(notification);
            navigate("coordination", notification.hash);
        } else {
            navigate("coordination", notification.hash);
        }
    }
</script>
<div
    style="padding: 20px;"
>
    <h1
        style="display: flex;"
    >Notifications
        <button
            class="mark-all-as-read-button"
            on:click={async () => {
                for (let i = 0; i < local_notifications.length; i++) {
                    if (!local_notifications[i].seen) {
                        await seeNotification(local_notifications[i]);
                    }
                }
            }}
        >Mark all as read</button>
    </h1>
    <ul id="notifications">
    {#each local_notifications as n}
        {#if !n.seen}
            <li on:click={() => goToCoordination(n)}><b>{n.description}</b></li>
        {:else}
            <!-- <li on:click={() => goToCoordination(n)}>{n.description}</li> -->
        {/if}
    {/each}
    </ul>
</div>

<style>
    h1 {
        font-size: 1.5rem;
        font-weight: 200;
        font-family: "Roboto", "Montserrat", sans-serif;
        color: #777777;
        display: flex;
    }

    .mark-all-as-read-button {
        margin-left: 14px;
        border: 0; 
        background: #5baaff; 
        border-radius: 4px; 
        cursor: pointer;
        color: white; 
        padding: 4px 8px;    
    }
    .mark-all-as-read-button:hover {
        background: #85bfff;
    }
</style>