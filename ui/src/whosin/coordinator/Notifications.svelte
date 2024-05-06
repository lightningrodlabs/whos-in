<script lang="ts">
    import { onMount, setContext, getContext } from 'svelte';
    import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
    import { clientContext } from '../../contexts';
    import type { Coordination, CoordinatorSignal } from './types';
    import { decode } from '@msgpack/msgpack';
    import { WeClient, isWeContext, initializeHotReload, type WAL, type Hrl } from '@lightningrodlabs/we-applet';  
    import { notifications, add_notification } from '../../store.js';
    import { appletServices } from '../../we';
    
    let client: AppAgentClient = (getContext(clientContext) as any).getClient();
    // export let client: AppAgentClient;
    
    let coordinations: Array<ActionHash> | undefined;
    let coordination_details = [];
    let local_notifications;
    let loading = true;
    let error: any = undefined;
    let unseen_notifications = 0;
    let weClient: WeClient

    // let unseen_notifications = 0;
    // let next_unseen_notifications = 0;
    
    $: coordinations, loading, error, unseen_notifications;
    
    onMount(async () => {
        // let test = "";
        weClient = await WeClient.connect(appletServices);

        if (typeof client != "undefined") {
            setInterval( () => {
                fetchNotifications();
            }, 30000);
        }
        else {
            // alert(client)
            // console.log(client)
        }
    
        await fetchNotifications();
    });

    notifications.subscribe(value => {
        local_notifications = value;
        unseen_notifications = value.filter(notification => notification.seen == false).length;
    });

    async function fetchCoordination(coordination) {
        loading = true;
        error = undefined;
        let record = undefined;
        let formatted_coordination = await decode((coordination.entry as any).Present.entry) as Coordination;
        let coordination_hash = coordination.signed_action.hashed.hash;
        // console.log(formatted_coordination.title)

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
                    let seenBool = false;
                    let seen = await client
                    .callZome({
                        cap_secret: null,
                        role_name: 'whosin',
                        zome_name: 'coordinator',
                        fn_name: 'find_coordination_links_for_viewer',
                        payload: coordination_hash,
                    });

                    if (seen % 2 != 0) {
                        seenBool = true;
                    }
                    
                    let new_notification = 
                        {
                            "timestamp": record[0].coordrole.signed_action.hashed.content.timestamp,
                            "type": "coordination-activation",
                            "description": "The "  + Object.keys(formatted_coordination.coordination_type)[0].toLocaleLowerCase() + " " + formatted_coordination.title + " has reached minimum participation",
                            "hash": coordination_hash,
                            "seen": seen,
                        }

                    console.log("----------------------------------", new_notification, local_notifications)
                    // if new_notification not in coordination_details yet, add
                    let alread_in = false;
                    local_notifications.forEach(notification => {
                        if (notification.timestamp == new_notification.timestamp) {
                            alread_in = true;
                        }
                    })
                    console.log("alread_in", alread_in)
                        
                    if (!alread_in) {
                        add_notification(new_notification)

                        if (new_notification.seen == 0) {
                            weClient.notifyFrame([{
                                title: "Coordination Activated",
                                body: new_notification.description,
                                notification_type: "change",
                                icon_src: undefined,
                                urgency: "high",
                                timestamp: Date.now()
                            }])
                        }
                    }
                            
                    // return seenBool
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

            // coordinations = records//.map(r => r.signed_action.hashed.hash);
            // coordinations = records.filter((v, i, a) => a.findIndex(t => JSON.stringify(t) === JSON.stringify(v)) === i);
            coordinations = records;

            // coordination_details = [];
            // next_unseen_notifications = 0;
            // let items_processed = 0;

            for (const coordination of coordinations) {
                await fetchCoordination(coordination);
            }
            // coordinations.forEach(c => {
            //     fetchCoordination(c);
                // let seen = fetchCoordination(c);
                // seen.then(function(result) {
                    // if (result == false) {
                        // next_unseen_notifications += 1;
                    // }
                    // items_processed += 1;
                    // if (items_processed === coordinations.length) {
                    //     unseen_notifications = next_unseen_notifications;
                    // }
                // });
            // })

        } catch (e) {
            error = e;
        }
        loading = false;
    }
    

</script>

{#if unseen_notifications > 0}
<div id="notifications-count">
    {unseen_notifications}
</div>
{/if}