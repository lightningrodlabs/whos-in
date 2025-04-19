<script lang="ts">
    import { onMount, createEventDispatcher } from 'svelte';
    import { writable } from 'svelte/store';
    import * as pluralize from 'pluralize';
    
    export let role;
    
    const dispatch = createEventDispatcher();
    function updateRole(field, value) {
        role[field] = value;
        dispatch('update', role);
    }

    let showDescription = false;
    let showRequiredNumber = false;
    let showLimitNumber = false;
    let showInviteList = false;
    let showSelectedMembers = false;
    let editingTitle = false;

    let title = '';
    let pluralizedTitle: 'joiners';
    $: if (title) { 
        updateRole('title', title); 
        try {
            pluralizedTitle = pluralize(title.toLowerCase())
        } catch (e) {
            console.log("Failed to pluralize title", e)
        }
    };
    let description = '';
    $: if (description) { updateRole('description', description); showDescription = true; };
    let requiredNumber = null;
    $: if (requiredNumber > 0) {  updateRole('minimum', requiredNumber); showRequiredNumber = true; };
    let limitNumber = null;
    $: if (limitNumber != null) {  updateRole('maximum', limitNumber); showLimitNumber = true };

    let inviteList = [];
    let membersList = [];
    let selectedMembers = [];

    // Mock data for members list
    onMount(() => {
        membersList = [
            { id: 1, name: 'Alice' },
            { id: 2, name: 'Bob' },
            { id: 3, name: 'Charlie' }
        ];

        title = role.title;
        description = role.description;
        requiredNumber = role.minimum;
        limitNumber = role.maximum;
        inviteList = role.inviteList;
        selectedMembers = role.selectedMembers;

        console.log("requiredNumber", requiredNumber, showRequiredNumber);
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

<div style="width: 100%">
        <div style="text-align: left;">
            <input class="title-input" placeholder="Role title" value={title || ""} on:input={e => { title = e.target.value; } } />
        </div>

        <div class="optional-field-outer">
            {#if showDescription}
                <button type="button" on:click={() => {showDescription = false; description = ""; }}>× description</button>
                <div class="optional-field">
                    <textarea id="description" placeholder="Description" bind:value={description}></textarea>
                </div>
            {:else}
                <button type="button" on:click={() => showDescription = true}>+ description</button>
            {/if}
        </div>

        <div class="optional-field-outer">
            {#if showRequiredNumber}
                <div class="optional-field" style="display: flex; align-items: center; gap: 8px;">
                    <button type="button" on:click={() => {
                        showRequiredNumber = false
                        requiredNumber = 0
                        updateRole('minimum', requiredNumber)
                    }}>× required {pluralizedTitle}: </button>
                    <input min="0" type="number" id="requiredNumber" bind:value={requiredNumber} style="width: auto;" placeholder={`Minimum ${pluralizedTitle}`} />
                </div>
            {:else}
                <button type="button" on:click={() => showRequiredNumber = true}>+ required {pluralizedTitle}</button>
            {/if}
        </div>

        <div class="optional-field-outer">
            {#if showLimitNumber}
                <div class="optional-field" style="display: flex; align-items: center; gap: 8px;">
                <button type="button" on:click={() => {
                    showLimitNumber = false
                    limitNumber = null
                    updateRole('maximum', limitNumber)
                }}>× maximum {pluralizedTitle}: </button>
                    <!-- <label for="limitNumber">Maximum number of {pluralizedTitle}:</label> -->
                    <input type="number" id="limitNumber" bind:value={limitNumber} min="1" />
                </div>
            {:else}
                <button type="button" on:click={() => { showLimitNumber = true; limitNumber = 1; }}>+ maximum {pluralizedTitle}</button>
            {/if}
        </div>

        <!-- <div class="optional-field-outer">
            {#if showInviteList}
                <button type="button" on:click={() => showInviteList = false}>× invite list</button>
                <div class="optional-field">
                    <label for="inviteList">Invite Specific People:</label>
                    <div style="display: flex; flex-direction: row; font-size: 12px;" >
                        <input type="checkbox" id="limitToInviteList" />
                        <label for="limitToInviteList">Limit signups to invite list</label>
                    </div>
                    <select id="inviteList" multiple size="5" on:change={handleInviteChange}>
                        {#each membersList as member}
                            <option value={member.id}>{member.name}</option>
                        {/each}
                    </select>
                </div>
            {:else}
                <button type="button" on:click={() => showInviteList = true}>+ invite list</button>
            {/if}
        </div> -->

        <!-- <button type="submit">Create Role</button> -->
</div>

<style>
    .title-input {
        background-color: #ffffff40;
        border: 0;
        outline: 0;
        font-size: 17px;
    }
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
        height: 10px;
    }

    input[type="number"] {
        width: 50px !important;
        background: transparent;
        border: 1px solid #939393;
        padding: 0.3em 0 0.3em 0.3em;
        margin: 0;
    }

    button {
        background: 0;
        border: 0;
        font-size: 16px;
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

    .optional-field-outer {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        /* gap: 1rem; */
        flex-wrap: wrap;
        width: 100%;
    }

    .optional-field {
        /* width: 100%;
        margin-bottom: 16px; 
        text-align: left;
        padding: 8px;
        margin: 0.2em 0;
        border-radius: 4px;
        background: #D5DAE540; */

        /* width: 100%; */
        margin-bottom: 16px;
        text-align: left;
        padding: 0;
        margin: 0;
        border-radius: 4px;
    }

    .optional-field label, .optional-field textarea, .optional-field input, .optional-field select {
        flex: 1;
    }
</style>