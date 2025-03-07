<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import type { NetworkInfoRequest } from '@holochain/client';
    import Matrix from './Matrix.svelte';
    import SvgIcon from './SvgIcon.svelte';
    import { averageColor } from "./crud/localStorage";

    export let client;

    const timeoutInterval = 60000;

    let lastMoved = new Date().getTime();
    let collapsed = false;
    let syncInterval;
    let networkInfo;
    let diff = 0;

    function detectCursorMovement() {
        lastMoved = new Date().getTime();
        diff = 0;
    }

    async function checkSync() {
        const now = new Date().getTime();
        diff = now - lastMoved;

        if (!collapsed && diff < timeoutInterval) {
            let networkInfoRequest: NetworkInfoRequest = {
                agent_pub_key: client.myPubKey,
                dnas: client.cachedAppInfo.cell_info.whosin.map((cell) => cell.provisioned.cell_id[0])
            }
            networkInfo = await client.networkInfoRequester(networkInfoRequest);
        }
    }

    onMount(() => {
        detectCursorMovement();
        checkSync();
        window.addEventListener('mousemove', detectCursorMovement);
        syncInterval = setInterval(checkSync, 10000);
    });

    onDestroy(() => {
        window.removeEventListener('mousemove', detectCursorMovement);
        clearInterval(syncInterval);
    });
</script>

{#if $averageColor}
<div class="sync-box" class:collapsed={collapsed}>
    <button class="toggle-button" on:click={() => collapsed = !collapsed}>
        {#if collapsed}▲ Network{:else}▼ Network{/if}
    </button>
    <div>
        {#if networkInfo?.[0]?.fetch_pool_info?.op_bytes_to_fetch > 0}
            Incoming data...
            <Matrix />
        {:else if diff > timeoutInterval}
            <span>
                Move cursor to check network
            </span>
        {:else if networkInfo?.[0]?.current_number_of_peers == 1}
            Alone in network
        {:else}
            <SvgIcon size=12 color="white" icon="faCheck" />
            <span
                style="margin-left: -8px;"
            >
                In sync with 
                <br>{networkInfo?.[0]?.current_number_of_peers} peers
            </span>
        {/if}
    </div>
</div>
{/if}


<style>
    .sync-box {
        width: 120px;
        position: fixed;
        bottom: 0;
        left: 0;
        /* background-color: rgba(105, 107, 110, 0.5); */
        background-color: var(--muted);
        backdrop-filter: blur(10px);
        border: 1px solid var(--dark-muted);
        color: white;
        padding: 10px;
        z-index: 1000;
        transition: transform 0.3s ease-in-out;
    }

    :global(.dark-mode ) .sync-box {
        /* background-color: rgba(255, 255, 255, 0.5); */
        color: white;
    }

    .collapsed {
        transform: translateY(100%);
    }

    .toggle-button {
        position: absolute;
        left: 30px;
        top: -28px;
        /* background-color: #007bff; */
        background-color: var(--dark-muted);
        color: white;
        border: none;
        padding: 6px;
        cursor: pointer;
    }
</style>