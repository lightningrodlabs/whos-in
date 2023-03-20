<script lang="ts">
    import { onMount, setContext, getContext } from 'svelte';
    import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
    import { clientContext } from '../../contexts';
    import type { Coordination, CoordinatorSignal } from './types';
    import { decode } from '@msgpack/msgpack';
    import { notifications, notifications_update } from '../../store.js';
    
    // let client: AppAgentClient = (getContext(clientContext) as any).getClient();
    export let client: AppAgentClient;
    
    let coordinations: Array<ActionHash> | undefined;
    let coordination_details = [];
    let local_notifications;
    let loading = true;
    let error: any = undefined;
    
    $: coordinations, loading, error;
    
    onMount(async () => {
        // let test = "";
        if (typeof client != "undefined") {
            setInterval( () => {
                fetchNotifications();
            }, 10000);
        }
        else {
            // alert(client)
            console.log(client)
        }
    
        await fetchNotifications();
    });

    notifications.subscribe(value => {
        local_notifications = value;
    });

    async function fetchCoordination(coordination) {
        loading = true;
        error = undefined;
        let record = undefined;
        let formatted_coordination = decode((coordination.entry as any).Present.entry) as Coordination;
        let coordination_hash = coordination.signed_action.hashed.hash;

        try {
            record = await client
            .callZome({
            cap_secret: null,
            role_name: 'whosin',
            zome_name: 'coordinator',
            fn_name: 'get_coordroles_for_coordination',
            payload: coordination_hash,
            });
            if (record) {
                let activated = true;

                record.forEach(role => {
                    if (role.participants < decode(role.coordrole.entry.Present.entry)["minimum"]) {
                        activated = false;
                    }
                })

                if (activated) {
                    coordination_details.push(
                        {
                            "type": "coordination-activation",
                            "description": "The Action " + formatted_coordination.title + " has reached minimum participation",
                            "hash": coordination_hash
                        }
                    );
                    notifications_update(coordination_details)
                }

            }

        } catch (e) {
            error = e;
        }
    }

    async function fetchNotifications() {
        try {
            const records = await client
            .callZome({
                cap_secret: null,
                role_name: 'whosin',
                zome_name: 'coordinator',
                fn_name: 'get_my_coordinations',
                payload: null,
            });

            coordinations = records//.map(r => r.signed_action.hashed.hash);
            coordination_details = [];

            coordinations.forEach(c => {
                fetchCoordination(c)
            })
            // coordination_details_show = coordination_details;
        } catch (e) {
            error = e;
        }
        loading = false;
    }
    

</script>

{#if local_notifications.length}
<div id="notifications-count">
    {local_notifications.length}
</div>
{/if}