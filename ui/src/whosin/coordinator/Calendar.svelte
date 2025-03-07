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
    import isEqual from 'fast-deep-equal';
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

    let calendarObject;
    let showEventModal = false;
    let refreshBoolean = true;
    let displayedEventHash = null;
    let showModalOpen = false;
    let createModalOpen = false;
    let selectedDateStart = new Date().valueOf() * 1000;
    let selectedDateEnd = (new Date(selectedDateStart).getTime() + 60 * 60 * 60 * 1000).valueOf() * 1000;
    let eventsList = []
    let userAvailability = {
        '1': [
            {
                status: 1,
                time: new Date().getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 2 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            // unavailable two days later
            {
                status: 0,
                time: new Date(new Date().getTime() + 6 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
        ],
        '2': [
            {
                status: 1,
                time: new Date().getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0.5,
                time: new Date(new Date().getTime() + 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            // random times throughout month
            {
                status: 0,
                time: new Date(new Date().getTime() + 2 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0,
                time: new Date(new Date().getTime() + 3 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 4 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0.5,
                time: new Date(new Date().getTime() + 5 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 6 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0.5,
                time: new Date(new Date().getTime() + 7 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 8 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0.5,
                time: new Date(new Date().getTime() + 9 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 10 * 24 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
        ],
        '3': [
            {
                status: 1,
                time: new Date().getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0.5,
                time: new Date(new Date().getTime() + 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 2 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0.5,
                time: new Date(new Date().getTime() + 3 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 4 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0.5,
                time: new Date(new Date().getTime() + 5 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 6 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0.5,
                time: new Date(new Date().getTime() + 7 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 1,
                time: new Date(new Date().getTime() + 8 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0,
                time: new Date(new Date().getTime() + 9 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
            {
                status: 0,
                time: new Date(new Date().getTime() + 10 * 60 * 60 * 1000).getTime(),
                duration: 60 * 60 * 1000,
            },
        ],
    }
    let availabilityList = [
        {
            id: '1',
            title: '👍2  🤷‍♀️0 🆇6',
            description: 'everyone is available',
            start: new Date().toISOString(),
            end: new Date(new Date().getTime() + 60 * 60 * 1000).toISOString(),
            display: 'auto',
            editable: false,
            allDay: false,
            backgroundColor: '#00800052;',
        }
    ]
    let availabilitySettings = {
        duration: 1 * 60 * 60 * 1000,
        participants: [],
    };

    function getUserAvailability(userId, timeSlotStart, slotDuration): number {
        let userAvailabilities = userAvailability[userId];
        let timeSlotEnd = timeSlotStart + slotDuration;
        let overlaps = userAvailabilities.filter(availability => {
            let availabilityStarts = availability.time;
            let availabilityEnds = availability.time + availability.duration;
            // any overlap between the availability and the time slot
            return availabilityStarts < timeSlotEnd && availabilityEnds > timeSlotStart;
        });
        if (overlaps.length === 0) {
            return 0.5;
        }
        let lowestAvailability = overlaps.reduce((a, b) => Math.min(a, b.status), 1);
        // console.log("lowest availability", timeSlotStart, slotDuration, lowestAvailability, timeSlotEnd, overlaps);
        return lowestAvailability;
    }

    function getUserAvailabilities(userIds, time, duration) {
        let allNumbers = [];
        let output = {
            average: 0,
            availableUsers: [],
            unavailableUsers: [],
            maybeAvailableUsers: [],
        }
        userIds.forEach(userId => {
            let availability = getUserAvailability(userId, time, duration);
            allNumbers.push(availability);
            if (availability === 1) {
                output.availableUsers.push(userId);
            } else if (availability === 0) {
                output.unavailableUsers.push(userId);
            } else {
                output.maybeAvailableUsers.push(userId);
            }
            output.average = allNumbers.reduce((a, b) => a + b, 0) / allNumbers.length;
        });
        return output;
    }

    function applyAvailaibility() {
        availabilityList = [];
        let userIds = ['1', '2', '3']//availabilitySettings.participants;
        let date = calendarObject.getOption('date');
        let month = date.getMonth();
        let year = date.getFullYear();
        let firstMonthDay = new Date(year, month, 1).getTime();
        let lastMonthDay = new Date(year, month + 1, 0).getTime();
        // availabilityChangeTimes is 15 minute increments for the entire month viewed
        let checkSlots = [];
        for (let i = firstMonthDay; i < lastMonthDay; i += 15 * 60 * 1000) {
            checkSlots.push(i);
        }
        let lastAvailabilities = null;
        let latestAvailability = null;
        checkSlots.forEach(slot => {
            let availabilities = getUserAvailabilities(userIds, slot, availabilitySettings.duration);
            if (latestAvailability) {
                latestAvailability.end = new Date(slot + availabilitySettings.duration);
            }
            // console.log("availabilities", availabilities);
            if (isEqual(availabilities, lastAvailabilities)) {
                return;
            }
            lastAvailabilities = availabilities;
            // let title = `👍${availabilities.availableUsers.length}  🤷‍♀️${availabilities.maybeAvailableUsers.length} 🆇${availabilities.unavailableUsers.length}`;
            let title = `Y:${availabilities.availableUsers.length}  M:${availabilities.maybeAvailableUsers.length} N:${availabilities.unavailableUsers.length}`;
            if (latestAvailability) {
                availabilityList.push(latestAvailability);
            }
            latestAvailability = {
                id: slot.toString(),
                title: title,
                description: '',
                start: new Date(slot).toISOString(),
                end: new Date(slot + availabilitySettings.duration).toISOString(),
                display: 'auto',
                editable: false,
                allDay: false,
                // red/yellow/green
                backgroundColor: `rgba(${255 * (1 - availabilities.average)}, ${255 * availabilities.average}, ${255 * availabilities.average + 100}, 0.5)`,
            }
        });
        if (latestAvailability) {
            availabilityList.push(latestAvailability);
        }
    }

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
            availabilityDetailsButton: {
                text: "everyone for 2 hours",
                active: true,
                click: function() {
                    options.customButtons.availabilityDetailsButton.active = true
                    showModalOpen = true;
                }
            },
            myAvailabilityButton: {
                text: "my availability",
                active: false,
                click: function() {
                    options.customButtons.myAvailabilityButton.active = true
                    options.customButtons.findaTimeButton.active = false
                    options.customButtons.eventsButton.active = false
                    options.headerToolbar.center = 'eventsButton,myAvailabilityButton,findaTimeButton'
                    refresh()
                }
            },
            findaTimeButton: {
                text: "find a time",
                active: false,
                click: function() {
                    options.customButtons.findaTimeButton.active = true
                    options.customButtons.eventsButton.active = false
                    options.customButtons.myAvailabilityButton.active = false
                    options.headerToolbar.center = 'eventsButton,myAvailabilityButton,findaTimeButton availabilityDetailsButton'
                    applyAvailaibility();
                    options.events = availabilityList;
                    refresh();
                }
            },
            eventsButton: {
                text: 'events',
                active: true,
                click: function() {
                    options.customButtons.eventsButton.active = true
                    options.customButtons.findaTimeButton.active = false
                    options.customButtons.myAvailabilityButton.active = false
                    options.headerToolbar.center = 'eventsButton,myAvailabilityButton,findaTimeButton'
                    options.events = eventsList;
                    refresh();
                }
            },
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
            },
        },
        headerToolbar: {
            start: 'monthButton,weekButton,dayButton,listButton',
            center: 'eventsButton,myAvailabilityButton,findaTimeButton',
            end: 'title, today prev,next'
        }
    };

    allCoordinationsDetails.subscribe(value => {
        eventsList = Object.keys(value)
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
        options.events = eventsList;
    });

    function refresh() {
        console.log("refreshing")
        refreshBoolean = false;
        setTimeout(() => {
            refreshBoolean = true;
        }, 0);
    }

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
            if (e.key === 'Escape') {
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
            <CreateCoordination agreementType="event" endsDate={null} startsDate={selectedDateStart} fromCalendar={true}
                on:coordination-created={async () => {
                    createModalOpen = false
                    await refetchCoordinationsWithDetails(weClient);
                    await refetchMyCoordinations(weClient);
                }}
                on:coordination-canceled={() => createModalOpen = false}
            />
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
    {#if refreshBoolean}
    <Calendar {plugins} {options} bind:this={calendarObject} />
        {/if}
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
        background: rgba(255, 255, 255, 0.5);
        backdrop-filter: blur(10px);
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
        body.dark-mode, body.dark-mode .ec-button, body.dark-mode .ec-title) {
        border-color: white;
        color: white;
    }
    :global(body.dark-mode .ec-button:hover) {
        background-color: #7c7c7c;
        color:white
    }
    :global(body.dark-mode .ec-button.ec-active) {
        background-color: #686868;
    }
    /* if no dark mode, make black */
    :global(.ec-header, .ec-all-day, .ec-body, .ec-days, .ec-day, .ec-day-head) {
        /* border-width: 2px; */
        /* border-color: color-mix(in srgb, white 50%, black 50%); */
        /* font-weight: bold; */
        /* color: color-mix(in srgb, white 50%, black 50%); */
        color: black;
        border-color: black;
    }
    :global(.dark-mode .ec-button, .dark-mode .ec-body, .dark-mode .ec-days) {
        background-color: rgba(0, 0, 0, 0.113);
    }
    :global(.ec-button, .ec-body, .ec-days) {
        background-color: rgba(255, 255, 255, 0.113);
        backdrop-filter: blur(10px);
    }
</style>
