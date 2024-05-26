<script lang="ts">
    import { onMount, getContext } from 'svelte';
    import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
    import { clientContext } from '../../contexts';
    import { notifications, navigate, setSeenNotification } from '../../store.js';
    
    let client: AppClient = (getContext(clientContext) as any).getClient();
    
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

    async function claimNotifier() {
        try {
        const record: Record = await client.callZome({
            cap_secret: null,
            role_name: 'whosin',
            zome_name: 'coordinator',
            fn_name: 'claim_notifier',
            payload: null,
        });

        } catch (e) {
            console.log(e)
        }
    }

    async function notifierPopup() {
        navigate("notifier");
    }
</script>

<!-- <button on:click={() => {notifierPopup()}}>Claim Notifier</button> -->
<!-- <button on:click={() => sendText()}>Send Text</button> -->

<!-- { JSON.stringify(error) } -->
<div class="invisible-outer">
    <h1
        style="display: flex;"
    >Notifications
        <button
            class="mark-all-as-read-button"
            on:click={() => {
                local_notifications.forEach(n => {
                    if (!n.seen) {
                        console.log(n)
                        seeNotification(n);
                    }
                });
            }}
        >Mark all as read</button>
    </h1>
    <ul id="notifications">
    {#each local_notifications as n}
        {#if !n.seen}
            <li on:click={() => goToCoordination(n)}><b>{n.description}</b></li>
        {:else}
            <li on:click={() => goToCoordination(n)}>{n.description}</li>
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