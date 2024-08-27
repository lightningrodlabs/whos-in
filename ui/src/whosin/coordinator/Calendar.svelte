<script lang="ts">
    import Calendar from '@event-calendar/core';
    import TimeGrid from '@event-calendar/time-grid';
    import DayGrid from '@event-calendar/day-grid';
    import ResourceTimeline from '@event-calendar/resource-timeline';
    import ResourceTimeGrid from '@event-calendar/resource-time-grid';
    import Interaction from '@event-calendar/interaction';
    import List from '@event-calendar/list';
    import { allCoordinations, allCoordinationsDetails, myCoordinations } from '../../crud/dataStore';
    import { refetchCoordinationsWithDetails, refetchMyCoordinations } from '../../crud/refetch';
    import { weClientStored } from '../../store';
    import { onMount, getContext } from 'svelte';
    import { clientContext } from '../../contexts';
    import type { AppClient } from '@holochain/client';
    import { getCoordinationLabel } from '../../util';
    import { get } from 'svelte/store';

    let myCoordinationsHashes;
    myCoordinations.subscribe(value => {
        myCoordinationsHashes = value;
    });

    let weClient;
    weClientStored.subscribe(value => {
        weClient = value;
    });

    let client: AppClient = (getContext(clientContext) as any).getClient();

    let plugins = [TimeGrid, ResourceTimeline, ResourceTimeGrid, Interaction, List, DayGrid];
    let options = {
        view: "dayGridMonth",
        events: [],
        customButtons: {
            listButton: {
                text: 'list',
                active: false,
                click: function() {
                    options.view = 'listYear';
                    options.customButtons.listButton.active = true;
                    options.customButtons.weekButton.active = false;
                    options.customButtons.dayButton.active = false;
                    options.customButtons.monthButton.active = false;
                }
            },
            dayButton: {
                text: 'day',
                active: false,
                click: function() {
                    options.view = 'timeGridDay';
                    options.customButtons.dayButton.active = true;
                    options.customButtons.weekButton.active = false;
                    options.customButtons.listButton.active = false;
                    options.customButtons.monthButton.active = false;
                }
            },
            weekButton: {
                text: 'week',
                active: false,
                click: function() {
                    options.view = 'timeGridWeek';
                    options.customButtons.weekButton.active = true;
                    options.customButtons.dayButton.active = false;
                    options.customButtons.listButton.active = false;
                    options.customButtons.monthButton.active = false;
                }
            },
            monthButton: {
                text: 'month',
                active: true,
                click: function() {
                    options.view = 'dayGridMonth';
                    options.customButtons.monthButton.active = true;
                    options.customButtons.weekButton.active = false;
                    options.customButtons.dayButton.active = false;
                    options.customButtons.listButton.active = false;
                }
            }
        },
        headerToolbar: {
            start: 'monthButton,weekButton,dayButton,listButton',
            center: 'title',
            end: 'today prev,next'
        }
    };

    allCoordinationsDetails.subscribe(value => {
        let eventList = Object.keys(value)
        .filter(key => value[key].starts_date && value[key].title)
        .map(key => {
            const event = value[key];
            console.log("EVENT LABEL", myCoordinationsHashes, key, myCoordinationsHashes.includes(key))
            return {
                id: key,
                title: event.title,
                description: event.description || '',
                start: new Date(event.starts_date / 1000).toISOString(),
                end: event.ends_date ? new Date(event.ends_date / 1000).toISOString() : null,
                editable: false,
                display: myCoordinationsHashes.some(item => item.coordinationHash === key) ? 'auto' : 'ghost',
                backgroundColor: getCoordinationLabel(event).color,
            };
        });
        console.log("eventList", eventList);
        options.events = eventList;
    });

    onMount(async () => {
        if (weClient.renderInfo.applets) {
            weClient.renderInfo.applets.forEach(applet => {
                console.log("applet client", applet);
                refetchCoordinationsWithDetails(applet.appletClient);
                refetchMyCoordinations(applet.appletClient);
            });
        } else {
            await refetchCoordinationsWithDetails(client);
            await refetchMyCoordinations(client);
        }
    });
</script>

<div style="
    padding: 16px;
">
    <Calendar {plugins} {options} />
</div>