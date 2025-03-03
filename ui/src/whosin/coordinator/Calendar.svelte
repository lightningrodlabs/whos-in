<script lang="ts">
    import Calendar from '@event-calendar/core';
    import TimeGrid from '@event-calendar/time-grid';
    import DayGrid from '@event-calendar/day-grid';
    import ResourceTimeline from '@event-calendar/resource-timeline';
    import ResourceTimeGrid from '@event-calendar/resource-time-grid';
    import Interaction from '@event-calendar/interaction';
    import List from '@event-calendar/list';
    import CreateCoordination from './Creation/CreateCoordination.svelte';
    import CoordinationDetail from './CoordinationDetail.svelte';
    import { allCoordinations, allCoordinationsDetails, myCoordinations } from '../../crud/dataStore';
    import { refetchCoordinationsWithDetails, refetchMyCoordinations } from '../../crud/refetch';
    import { weClientStored, navigate } from '../../store';
    import { onMount, getContext } from 'svelte';
    import { clientContext } from '../../contexts';
    import type { AppClient } from '@holochain/client';
    import { getCoordinationLabel } from '../../util';
    import { get } from 'svelte/store';
    import { decodeHashFromBase64 } from '@holochain/client';

    let myCoordinationsHashes;
    myCoordinations.subscribe(value => {
        myCoordinationsHashes = value;
    });

    let weClient;
    weClientStored.subscribe(value => {
        weClient = value;
    });

    let backupClient: AppClient = (getContext(clientContext) as any).getClient();

    let showEventModal = false;
    let displayedEventHash = null;
    let showModalOpen = false;
    let createModalOpen = false;
    let selectedDateStart = new Date().valueOf() * 1000;
    let selectedDateEnd = (new Date(selectedDateStart).getTime() + 60 * 60 * 60 * 1000).valueOf() * 1000;

    let plugins = [TimeGrid, ResourceTimeline, ResourceTimeGrid, Interaction, List, DayGrid];
    let options = {
        view: "dayGridMonth",
        events: [],
        eventClick: function(info) {
        //    navigate("coordination", decodeHashFromBase64(info.event.id));
            displayedEventHash = decodeHashFromBase64(info.event.id);
            showEventModal = true;
        },
        dateClick: function(info) {
            console.log(info)
            selectedDateStart = new Date(info.date).valueOf() * 1000
            if (info.allDay) {
                selectedDateEnd = new Date(info.date).valueOf() * 1000 + 24 * 60 * 60 * 1000;
                console.log("Selected date end:", new Date(selectedDateEnd / 1000).toLocaleString());
            } else {
                selectedDateEnd = new Date(info.date).valueOf() * 1000 + 60 * 60 * 60 * 1000;
            }
            createModalOpen = true;
        },
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
            return {
                id: key,
                title: event.title,
                description: event.description || '',
                start: new Date(event.starts_date / 1000).toISOString(),
                end: event.ends_date ? new Date(event.ends_date / 1000).toISOString() : new Date(event.starts_date / 1000).toISOString(),
                editable: false,
                allDay: false,
                display: 'auto', // myCoordinationsHashes.some(item => item.coordinationHash === key) ? 'auto' : 'ghost',  
                backgroundColor: getCoordinationLabel(event).color,
            };
        });
        options.events = eventList;
    });

    onMount(async () => {
        if (weClient?.renderInfo.applets) {
            weClient?.renderInfo.applets.forEach(applet => {
                refetchCoordinationsWithDetails(applet.appletClient);
                refetchMyCoordinations(applet.appletClient);
            });
        } else {
            await refetchCoordinationsWithDetails(backupClient);
            await refetchMyCoordinations(backupClient);
        }

        // close modal on esc
        window.addEventListener('keydown', (e) => {
            console.log("key", e.key)
            if (e.key === 'Escape') {
                console.log("excape")
                createModalOpen = false;
                showEventModal = false;
                showModalOpen = false;
            }
        });
    });
</script>

{#if createModalOpen}
    <div class="modal-overlay" on:click={() => createModalOpen = false}>
        <div class="modal-content" on:click|stopPropagation>
            <button on:click={() => createModalOpen = false}>Close</button>
            <CreateCoordination agreementType="event" endsDate={null} startsDate={selectedDateStart} />
        </div>
    </div>
{/if}
{#if showModalOpen}
    <div class="modal-overlay" on:click={() => showModalOpen = false}>
        <div class="modal-content" on:click|stopPropagation>
            <button on:click={() => showModalOpen = false}>Close</button>
            <div style="padding: 16px;">
                <Calendar {plugins} {options} />
            </div>
        </div>
    </div>
{/if}

{#if showEventModal}
<div class="modal-overlay" on:click={() => showEventModal = false}>
    <div class="modal-content" on:click|stopPropagation>
        <button on:click={() => showEventModal = false}>Close</button>
        <CoordinationDetail coordinationHash={displayedEventHash} />
    </div>
</div>
{/if}

<div style="padding: 16px;">
<Calendar {plugins} {options} />
</div>

<style>
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 2;
    }
    .modal-content {
        background: white;
        padding: 20px;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        height: 80%;
        overflow-y: auto;
        justify-content: left;
    }
    .modal-content > button {
        position: absolute;
        top: 10px;
        right: 10px;
    }
    /* global class ec-bg-events*/
    :global(.ec-bg-events:hover) {
        cursor: pointer;
    }
    :global(.ec-bg-events:hover::after) {
        content: '+ new event';
        position: absolute;
        background-color: #cccccc;
        top: 88%;
        width: 97%;
        height: 20px;
        text-align: left;
        padding-top: 2px;
        padding-left: 3px;
        transform: translate(-50%, -50%);
        color: #ffffff;
        font-size: 14px;
    }
    :global(body.dark-mode .ec-header, body.dark-mode .ec-all-day, 
        body.dark-mode .ec-body, body.dark-mode .ec-days, 
        body.dark-mode .ec-day, body.dark-mode .ec-day-head,
        body.dark-mode) {
        border-color: white;
        color: white;
    }
    /* if no dark mode, make black */
    :global(.ec-header, .ec-all-day, .ec-body, .ec-days, .ec-day, .ec-day-head) {
        border-color: black;
        color: black;
    }
</style>
