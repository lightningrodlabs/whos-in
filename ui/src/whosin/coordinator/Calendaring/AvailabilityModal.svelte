<script lang="ts">
    import SvgIcon from "../../../SvgIcon.svelte";
    import { secondsToDateInput } from "../Creation/helper";
    import { getUserAvailability } from "./availability";
    import { encodeHashToBase64 } from "@holochain/client";
    export let selectedAvailability: any;
    export let client: any;
    // export let editing: boolean;
    // export let deleteAvailability: Function;
    export let addAvailabilities: Function;
    // export let updateAvailability: Function;
    export let showAvailabilityModal: boolean;

    let repeat: any = null;
    let repeatUntil: string = secondsToDateInput(selectedAvailability.time);
    let selectedTime: string = secondsToDateInput(selectedAvailability.time);
    let selectedEndTime: string = secondsToDateInput(selectedAvailability.time + 3600000);
</script>

    <strong class="modal-title">Add availability</strong>
    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>

<!-- {JSON.stringify(selectedAvailability)} -->

<!-- time, status fields -->
<div style="margin-bottom: 0.5rem;">
    <label for="time" class="form-label">Starting</label>

    <input type="datetime-local" id="start-date" name="start-date" 
        bind:value={selectedTime}
        required
    />
</div>
<div style="margin-bottom: 0.5rem;">
    <label for="end-time" class="form-label">Ending</label>
    <input type="datetime-local" id="end-date" name="end-date" 
        bind:value={selectedEndTime}
        required
    />
</div>
<div style="margin-bottom: 0.5rem;">
    <label for="status" class="form-label">Status</label>
    <select class="form-select" id="status" bind:value={selectedAvailability.status} on:change={(e) => selectedAvailability.status = parseFloat(e.target.value)}>
        <option value={1}>Available</option>
        <option value={0.5}>Tentative</option>
        <option value={0}>Unavailable</option>
    </select>
</div>

<!-- Repeat? day/week/month -->
<div class="mb-3">
    <label for="repeat" class="form-label">
        Repeat?
    </label>
    <select class="form-select" id="repeat" bind:value={repeat}>
        <option value={null}>No</option>
        <option value={'day'}>Every day</option>
        <option value={'week'}>Every week</option>
        <option value={'month'}>Every month</option>
    </select>
</div>
<!-- Repeat for how long? -->
{#if repeat}
<div style="margin-top: 0.5rem;">
    <label for="repeat-until" class="form-label">
            Repeat until
        </label>
        <input type="datetime-local" id="repeat-until" name="repeat-until" 
            bind:value={repeatUntil}
            required
        />
    </div>
{/if}

<div
    class="modal-footer"
    style="display: flex; justify-content: space-between; align-items: center; padding: 1rem 2rem 0 1rem;"
>
    <!-- {#if selectedAvailability}
        <button
            class="btn btn-danger"
            on:click={() => {
                deleteAvailability(selectedAvailability);
            }}
        >
            <SvgIcon icon="faTrash" />
            Delete
        </button>
    {/if} -->

    <!-- cancel button -->
    <button
        class="btn btn-danger"
        on:click={() => {
            showAvailabilityModal = false;
        }}
    >
        <SvgIcon color="#fff" icon="faClose" />
        Cancel
    </button>

    <!-- add button -->
    <button
        class="btn btn-primary"
        on:click={() => {
            // export function getUserAvailability(client, userId, timeSlotStart, slotDuration): number {
            let previousEndAvailability = getUserAvailability(
                client,
                encodeHashToBase64(client.myPubKey),
                new Date(selectedEndTime).getTime(),
                0
            );
            if (repeat) {
                let newAvailabilities = []
                let latestTime = new Date(selectedTime).getTime();
                let latestEndTime = new Date(selectedEndTime).getTime();
                console.log('repeateUntil', new Date(selectedTime), new Date(repeatUntil));
                console.log('repeateUntil', new Date(selectedTime).getTime(), new Date(repeatUntil).getTime());
                console.log((new Date(selectedTime).getTime() < new Date(repeatUntil).getTime()) )
                const repeatUntilTime = new Date(repeatUntil).getTime();
                let cycle = 0;
                while (cycle < 999 && latestTime < repeatUntilTime) {
                    cycle++;
                    console.log('latestTime', latestTime);
                    newAvailabilities.push({
                        ...selectedAvailability,
                        time: latestTime,
                    });
                    newAvailabilities.push({
                        status: previousEndAvailability,
                        time: latestEndTime,
                    });
                    if (repeat === 'day') {
                        latestTime = latestTime + 24 * 60 * 60 * 1000;
                        latestEndTime = latestEndTime + 24 * 60 * 60 * 1000;
                    }
                    else if (repeat === 'week') {
                        latestTime = latestTime + 7 * 24 * 60 * 60 * 1000;
                        latestEndTime = latestEndTime + 7 * 24 * 60 * 60 * 1000;
                    }
                    else if (repeat === 'month') {
                        latestTime = latestTime + 30 * 24 * 60 * 60 * 1000;
                        latestEndTime = latestEndTime + 30 * 24 * 60 * 60 * 1000;
                    }
                    else {
                        break;
                    }
                }
                console.log('newAvailabilities', newAvailabilities);
                addAvailabilities(newAvailabilities);
            }
            else {
                addAvailabilities([
                    {
                        ...selectedAvailability,
                        time: new Date(selectedTime).getTime(),
                    },
                    {
                        status: previousEndAvailability,
                        time: new Date(selectedEndTime).getTime(),
                    }
                ]);
            }
        }}
    >
        <SvgIcon color="#fff" icon="faPlus" />
        Add
    </button>
</div>

<style>
    .modal-title {
        font-size: 1.5rem;
        text-align: center;
        font-weight: bold;
        margin-bottom: 1rem;
    }
    .btn-close {
        display: none;
    }
    .btn-danger {
        background-color: #7a7a7a;
        color: white;
        border: none;
    }
    .btn-primary {
        background-color: #007bff;
        border: none;
        color: white;
    }
    .btn-primary:hover {
        background-color: #0056b3;
        border-color: #0056b3;
    }
    .btn-danger:hover {
        background-color: #3a3a3a;
    }
    .btn {
        margin: 0.5rem;
        padding: 0.5rem 1rem;
        font-size: 1rem;
        border-radius: 0.25rem;
        cursor: pointer;
        display: flex;
        align-items: center;
    }
    .btn:focus {
        outline: none;
        box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
    }
    .btn:active {
        transform: translateY(1px);
        box-shadow: none;
    }
    .btn:disabled {
        opacity: 0.65;
        cursor: not-allowed;
    }
    .form-label {
        font-weight: bold;
        margin-bottom: 0.5rem;
    }
    .form-select {
        width: 100%;
        padding: 0.5rem;
        border-radius: 0.25rem;
        border: 1px solid #ced4da;
        background-color: #fff;
        font-size: 1rem;
    }
    .form-select:focus {
        outline: none;
        box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
        border-color: #80bdff;
    }
    .form-select:disabled {
        background-color: #e9ecef;
        opacity: 1;
        cursor: not-allowed;
    }
    .form-select:disabled:hover {
        background-color: #e9ecef;
        border-color: #ced4da;
    }
    input[type="datetime-local"] {
        width: 92%;
        padding: 0.5rem;
        border-radius: 0.25rem;
        border: 1px solid #ced4da;
        background-color: #fff;
        font-size: 1rem;
    }
    </style>