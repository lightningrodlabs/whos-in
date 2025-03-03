<script lang="ts">
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';
    import * as pluralize from 'pluralize';

    export let role;

    let title = 'Participant';
    let description = '';
    let requiredNumber = null;
    let limitNumber = null;
    let inviteList = [];
    let membersList = [];
    let selectedMembers = [];

    let showDescription = false;
    let showRequiredNumber = false;
    let showLimitNumber = false;
    let showInviteList = false;
    let showSelectedMembers = false;
    let editingTitle = false;

    // Mock data for members list
    onMount(() => {
        membersList = [
            { id: 1, name: 'Alice' },
            { id: 2, name: 'Bob' },
            { id: 3, name: 'Charlie' }
        ];

        title = role.title;
        description = role.description;
        requiredNumber = role.requiredNumber;
        limitNumber = role.limitNumber;
        inviteList = role.inviteList;
        selectedMembers = role.selectedMembers;
    });

    function handleInviteChange(event) {
        const { options } = event.target;
        inviteList = Array.from(options).filter(option => option.selected).map(option => option.value);
    }

    function handleMembersChange(event) {
        const { options } = event.target;
        selectedMembers = Array.from(options).filter(option => option.selected).map(option => option.value);
    }

    function toggleTitleEdit() {
        editingTitle = !editingTitle;
    }
</script>

<div style="width: 70%">
        <div style="text-align: left;">
            <input class="title-input" placeholder="Role title" value={title || ""} on:input={e => { title = e.target.value; } } />
        </div>

        <div class="optional-field">
            {#if showDescription}
                <textarea id="description" placeholder="Description" bind:value={description}></textarea>
                <button type="button" on:click={() => showDescription = false}>Remove Description</button>
            {:else}
                <button type="button" on:click={() => showDescription = true}>+ Description</button>
            {/if}
        </div>

        <div class="optional-field">
            {#if showRequiredNumber}
                <label for="requiredNumber">Required number of {title ? pluralize(title.toLowerCase()) : "joiners"}:</label>
                <input type="number" id="requiredNumber" bind:value={requiredNumber} min="1" />
                <button type="button" on:click={() => showRequiredNumber = false}>Remove requirement</button>
            {:else}
                <button type="button" on:click={() => showRequiredNumber = true}>+ Participation requirement</button>
            {/if}
        </div>

        <div class="optional-field">
            {#if showLimitNumber}
                <label for="limitNumber">Maximum number of {title ? pluralize(title.toLowerCase()) : "joiners"}:</label>
                <input type="number" id="limitNumber" bind:value={limitNumber} min="1" />
                <button type="button" on:click={() => showLimitNumber = false}>Remove limit</button>
            {:else}
                <button type="button" on:click={() => showLimitNumber = true}>+ Participation limit</button>
            {/if}
        </div>

        <div class="optional-field">
            {#if showInviteList}
                <label for="inviteList">Invite Specific People:</label>
                <!-- limit signups to invite list? checkbox -->
                <input type="checkbox" id="limitToInviteList" />
                <label for="limitToInviteList">Limit signups to invite list</label>
                <select id="inviteList" multiple size="5" on:change={handleInviteChange}>
                    {#each membersList as member}
                        <option value={member.id}>{member.name}</option>
                    {/each}
                </select>
                <button type="button" on:click={() => showInviteList = false}>Remove list</button>
            {:else}
                <button type="button" on:click={() => showInviteList = true}>+ Invite list</button>
            {/if}
        </div>

        <!-- <button type="submit">Create Role</button> -->
</div>

<style>
    div {
        /* margin-bottom: 1rem; */
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
    }

    input, textarea, select {
        width: 100%;
        padding: 0.5rem;
        margin-bottom: 0.5rem;
    }

    button {
        background: 0;
        border: 0;
        /* padding: 0.5rem 1rem; */
    }
    
    button:hover {
        text-decoration: underline;
        cursor: pointer;
    }

    .editable-title {
        display: block;
        padding: 0.5rem;
        border: 1px solid #ccc;
        cursor: pointer;
        background-color: #f9f9f9;
    }

    .editable-title:hover {
        background-color: #e9e9e9;
    }

    .optional-field {
        display: flex;
        align-items: center;
        gap: 1rem;
        flex-wrap: wrap;
    }

    .optional-field label, .optional-field textarea, .optional-field input, .optional-field select {
        flex: 1;
    }
</style>