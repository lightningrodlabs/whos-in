<script lang="ts">
    import Calendar from '@event-calendar/core';
    import TimeGrid from '@event-calendar/time-grid';
    import DayGrid from '@event-calendar/day-grid';
    import ResourceTimeline from '@event-calendar/resource-timeline';
    import ResourceTimeGrid from '@event-calendar/resource-time-grid';
    import Interaction from '@event-calendar/interaction';
    import List from '@event-calendar/list';
    import CreateCoordination from '../Creation/CreateCoordination.svelte';
    import CoordinationDetail from '../CoordinationDetail.svelte';
    import { allCoordinations, allCoordinationsDetails, myCoordinations } from '../../../crud/dataStore';
    import { refetchCoordinationsWithDetails, refetchMyCoordinations } from '../../../crud/refetch';
    import { weClientStored, navigate } from '../../../store';
    import { onMount, getContext } from 'svelte';
    import { clientContext } from '../../../contexts';
    import type { AppClient } from '@holochain/client';
    import { getCoordinationLabel } from '../../../util';
    import { get } from 'svelte/store';
    import isEqual from 'fast-deep-equal';
    import { decodeHashFromBase64 } from '@holochain/client';
    import FindATimeSettings from './FindATimeSettings.svelte';
    import { getLocalISOString, reAddOffset } from './helper';
    import { fade } from 'svelte/transition';
    import { cloneDeep } from 'lodash';
    import { secondsToDateInput } from '../Creation/helper';
    import { debounce } from 'lodash';

    let applets: Array<any> = (getContext(clientContext) as any).getApplets();
    let client: AppClient = (getContext(clientContext) as any).getClient();

    const refresh = () => {
        console.log("refreshing");
        refreshBoolean = false;
        setTimeout(() => {
            refreshBoolean = true;
        }, 0);
    }

    const debouncedRefresh = debounce(() => {
        refresh();
    }, 300);


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
        let firstMonthHour = new Date(year, month, 1).getTime();
        let nextMonthHour = new Date(year, month + 1, 1).getTime();
        // availabilityChangeTimes is 15 minute increments for the entire month viewed
        let checkSlots = [firstMonthHour];
        for (let i = firstMonthHour; i < nextMonthHour; i += 15 * 60 * 1000) {
            checkSlots.push(i);
        }
        let lastAvailabilities = null;
        let latestAvailability = null;
        checkSlots.forEach(slot => {
            let availabilities = getUserAvailabilities(userIds, slot, availabilitySettings.duration);
            if (latestAvailability) {
                let endTime = new Date(slot + availabilitySettings.duration).getTime();
                /* if next day, cut off at midnight, otherwise add use duration */
                let latestDate = new Date(latestAvailability.end).getDate();
                let potentialDate = new Date(endTime).getDate();
                if (latestDate == potentialDate) {
                    let oneMinuteBeforeMidnight = new Date(new Date(slot).setHours(23, 59, 59, 0)).getTime();
                    latestAvailability.end = getLocalISOString(new Date(oneMinuteBeforeMidnight));
                }
            }
            if (isEqual(availabilities, lastAvailabilities) && 
            new Date(slot).toLocaleDateString() == new Date(latestAvailability?.start).toLocaleDateString()) {
                return;
            }
            lastAvailabilities = availabilities;
            let title = `☑️${availabilities.availableUsers.length} ❔${availabilities.maybeAvailableUsers.length} 🇽${availabilities.unavailableUsers.length}`;
            // let title = `Y:${availabilities.availableUsers.length}  M:${availabilities.maybeAvailableUsers.length} N:${availabilities.unavailableUsers.length}`;
            if (latestAvailability) {
                let endHour = new Date(slot).toLocaleString('en-US', { hour: 'numeric', minute: 'numeric', hour12: true });
                latestAvailability.title = " to " + endHour + `
` + latestAvailability.title //title + " - " + Math.round(availabilities.average * 1000) / 10000;
                availabilityList.push(latestAvailability);
            }
            const average = cloneDeep(availabilities.average) == 0.5 ? 0.5 : Math.round(availabilities.average * 1000) / 1000;
            const color = `rgba(${255 * (1 - average)}, ${255 * (average)}, ${255 * (100)}, 0.5)`;
            const startingEnd = new Date(slot + availabilitySettings.duration).getTime();
            const nextDayHour = new Date(new Date(slot).setHours(23, 59, 59, 0)).getTime();
            latestAvailability = {
                id: slot,
                title: title,
                description: '',
                start: getLocalISOString(new Date(slot)),
                end: getLocalISOString( startingEnd < nextDayHour ? new Date(startingEnd) : new Date(nextDayHour)),
                display: 'auto',
                editable: false,
                allDay: false,
                // red/yellow/green
                backgroundColor: color//`rgba(${255 * (1 - average)}, ${255 * (average)}, ${255 * (average + 100)}, 0.5)`,
            }
        });
        if (latestAvailability) {
            let endHour = new Date(nextMonthHour).toLocaleString('en-US', { hour: 'numeric', minute: 'numeric', hour12: true });
            latestAvailability.title = "to " + endHour + `
` + latestAvailability.title;
            availabilityList.push(latestAvailability);
        }
    }

    const openEventSlot = function(info) {
        console.log(info.event)
        selectedDateStart = new Date(info.event.start).getTime()
        selectedDateEnd = new Date(info.event.start).getTime() + availabilitySettings.duration
        createModalOpen = true;
    }

    const openEvent = function(info) {
        displayedEventHash = decodeHashFromBase64(info.event.id);
        showEventModal = true;
    }

    let plugins = [TimeGrid, ResourceTimeline, ResourceTimeGrid, Interaction, List, DayGrid];
    let options = {
        view: "dayGridMonth",
        events: [],
        eventClick: openEvent,
        dateClick: function(info) {
            // console.log(info)
            // console.log("add offset", reAddOffset(info.date))
            selectedDateStart = new Date(info.date).getTime() //reAddOffset(info.date) //new Date(info.date).valueOf() * 1000
            if (info.allDay) {
                selectedDateEnd = new Date(info.date).getTime() + (24 * 60 * 60 * 1000 - 1);
                console.log("Selected date end:", new Date(selectedDateEnd).toLocaleString());
            } else {
                selectedDateEnd = new Date(info.date).getTime() + (60 * 60 * 1000);
                console.log("Selected date ends at:", new Date(selectedDateEnd).toLocaleString());
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
                    options.events = []
                    document.body.classList.toggle("fat", false);
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
                    document.body.classList.toggle("fat", true);
                    options.eventClick = openEventSlot
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
                    document.body.classList.toggle("fat", false);
                    options.eventClick = openEvent
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
                // start: new Date(event.starts_date / 1000).toISOString(),
                start: getLocalISOString(new Date(event.starts_date / 1000)),
                end: event.ends_date ? getLocalISOString(new Date(event.ends_date / 1000)) : getLocalISOString(new Date(event.starts_date / 1000)),
                editable: false,
                allDay: false,
                display: 'auto', // myCoordinationsHashes.some(item => item.coordinationHash === key) ? 'auto' : 'ghost',  
                backgroundColor: getCoordinationLabel(event).color,
            };
        });
        if (options.customButtons.eventsButton.active) {
            options.events = eventsList;
        }
        debouncedRefresh();
    });

    onMount(async () => {
        console.log("=======================applets", applets)
        if (applets) {
                for (let i = 0; i < applets.length; i++) {
                    console.log("refetching applet", applets[i][1].appletClient)
                    await refetchCoordinationsWithDetails(applets[i][1].appletClient);
                }
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
    <div class="modal-overlay" in:fade={{duration: 40}} out:fade={{duration: 40}} on:mousedown={() => createModalOpen = false}>
        <div class="modal-content" on:mousedown|stopPropagation>
            <button on:click={() => createModalOpen = false}>×</button>
            <CreateCoordination agreementType="event" endsDate={selectedDateEnd} startsDate={selectedDateStart} fromCalendar={true}
                on:coordination-created={async () => {
                    createModalOpen = false
                    console.log("coordination created", applets)
                    if (applets) {
                        console.log("applets", applets)
                        for (let i = 0; i < applets.length; i++) {
                            console.log("refetching applet", applets[i][1].appletClient)
                            await refetchCoordinationsWithDetails(applets[i][1].appletClient);
                        }
                    } else {
                        console.log("refetching backup client")
                        await refetchCoordinationsWithDetails(client || backupClient);
                    }
                    refresh();
                }}
                on:coordination-canceled={() => createModalOpen = false}
            />
        </div>
    </div>
{/if}
{#if showModalOpen}
    <div class="modal-overlay" in:fade={{duration: 40}} out:fade={{duration: 40}} on:mousedown={() => showModalOpen = false}>
        <div class="modal-content" on:mousedown|stopPropagation>
            <button on:click={() => showModalOpen = false}>×</button>
            <div style="padding: 16px;">
                <FindATimeSettings />
                <!-- <Calendar {plugins} {options} /> -->
            </div>
        </div>
    </div>
{/if}

{#if showEventModal}
    <div class="modal-overlay" out:fade={{duration: 80}} on:mousedown={() => showEventModal = false}>
        <div class="modal-content" on:mousedown|stopPropagation>
            <button on:click={() => showEventModal = false}>×</button>
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
        /* height: 80%; */
        overflow-y: auto;
        justify-content: left;
        display: flex;
        flex-direction: column;
        text-align: left;
    }
    .modal-content > button {
        position: absolute;
        top: 0px;
        right: 0px;
        border: 0;
        padding: 3px 7px;
        border-radius: 6px;
        color: white;
        background: transparent;
        font-size: 16px;
        cursor: pointer;
    }
    .modal-content > button:hover {
        background: rgba(0, 0, 0, 0.1);
    }
    /* global class ec-bg-events*/
    :global(.ec-bg-events:hover) {
        cursor: pointer;
    }
    :global(.ec-bg-events:hover::after) {
        content: '+ new event';
        position: absolute;
        background-color: rgba(157, 157, 157, 0.686);
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
    :global(.dark-mode .ec-bg-events:hover::after) {
        background-color: rgba(204, 204, 204, 0.12);
    }
    :global(.fat .ec-bg-events:hover::after) {
        display: none;
    }
    :global(body.dark-mode .ec-header, body.dark-mode .ec-all-day, 
        body.dark-mode .ec-body, body.dark-mode .ec-days, 
        body.dark-mode .ec-day, body.dark-mode .ec-day-head,
        body.dark-mode, body.dark-mode .ec-button, body.dark-mode .ec-title) {
        border-color: white;
        color: white;
    }
    :global(body.dark-mode .ec-button:hover) {
        color:white
    }
    :global(.ec-button:hover) {
        background-color: calc(var(--light-vibrant)) !important;
    }
    :global(.dark-mode .ec-button:hover) {
        background-color: calc(var(--dark-vibrant)) !important;
    }
    :global(.ec-button.ec-active) {
        background-color: var(--light-vibrant) !important;
    }
    :global(.dark-mode .ec-button.ec-active) {
        background-color: var(--dark-vibrant) !important;
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
    :global(.ec-button-group, .ec-body, .ec-days) {
        background-color: rgba(255, 255, 255, 0.113);
        backdrop-filter: blur(10px);
    }

    /* only for find a time */
    :global(.fat .ec-event-body, .fat .ec-event-time, .fat .ec-event-title) {
        display: inline;
    }
    :global(.fat .ec-event-title) {
        white-space: pre-wrap !important;
    }
    :global(.ec-event:hover) {
        cursor: pointer;
        opacity: 0.6;
    }
    :global(article.ec-event) {
        min-height: fit-content !important;
        height: fit-content !important;
        /* width: 100% !important; */
        left: 3px !important;
    }
</style>
