<script lang="ts">
import Avatar from "../Avatar.svelte";
import { decodeHashFromBase64 } from "@holochain/client";
import { allAvailability } from "../../../crud/dataStore";
export let availabilityDuration;
export let availabilityParticipants;
export let applyAvailaibility: Function;
export let refresh;

let allUsers = []
allAvailability.subscribe((data) => {
    allUsers = Object.keys(data)
});

let displayedUsers = [...allUsers];

</script>

<h1>Find a time</h1>

<!-- form that provides a list of users and lets you check/uncheck and search-filter them -->
<form>
    <!-- input to determine a period of time period in 15-minute increments -->
    <div class="form-group"
        style="flex-direction: column; align-items: center;"
    >
        <div>
            <input type="number" id="hours" name="hours" min="0" max="24" 
                value={Math.floor(availabilityDuration / 3600000)} 
                on:input={(e) => {
                    availabilityDuration = ((parseInt(e.target.value) || 0) * 3600000) + ((parseInt(document.getElementById('minutes').value) || 0) * 60000);
                    console.log(availabilityDuration);
                    applyAvailaibility();
                    refresh();
                }}
            /> hours
            <input type="number" id="minutes" name="minutes" min="0" max="59" 
                value={(availabilityDuration % 3600000) / 60000} 
                on:input={(e) => {
                    availabilityDuration = ((parseInt(document.getElementById('hours').value) || 0) * 3600000) + ((parseInt(e.target.value) || 0) * 60000);
                    applyAvailaibility();
                    refresh();
                }}
            /> minutes
        </div>
    </div>

    <!-- <div class="form-group">
        <input type="text" placeholder="Search for users..." 
        style="margin-right: 1em;"
            on:input={(e) => {
                displayedUsers = allUsers.filter(u => u.toLowerCase().includes(e.target.value.toLowerCase()));
            }}
        />
        <button type="button" on:click={() => {
            if (selectedUsers.length === allUsers.length) {
            selectedUsers = [];
            } else {
            selectedUsers = [...allUsers];
            }
        }}>
            {selectedUsers.length === allUsers.length ? 'unselect all' : 'select all'}
        </button>
    </div> -->
  <div class="user-list">
    {#each displayedUsers as user}
      <div class="form-group-checkbox">
        <input type="checkbox" id={user} name={user} value={user} checked={availabilityParticipants.includes(user)}
            on:change={async () => {
                if (availabilityParticipants.includes(user)) {
                    availabilityParticipants = availabilityParticipants.filter(u => u !== user);
                } else {
                    availabilityParticipants = [...availabilityParticipants, user];
                }
                applyAvailaibility();
                refresh();
            }}
        />
        <label class="avatar-label" for={user}>
            <Avatar showNickname={true} agentPubKey={decodeHashFromBase64(user)} size={24} namePosition="row"></Avatar>
        </label>
      </div>
    {/each}
  </div>

  <!-- apply -->
    <!-- <button type="submit" 
    style="margin-top: 1em;"
    on:click={(e) => {
        e.preventDefault();
        console.log(availabilityParticipants);
    }}>Apply</button> -->
</form>

<style>
h1 {
    margin-bottom: 16px;
    margin-top: 0;
    text-align: center;
}

  form {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  input[type="checkbox"] {
    margin-right: 8px;
  }
.form-group {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin-bottom: 8px;
}
.form-group-checkbox {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin: 4px 0;
}
.avatar-label {
    width: 100%;
    padding: 4px 0;
    cursor: pointer;
}
.user-list {
    max-height: 300px; 
    overflow-y: auto;
    border: 1px solid gray;
    width: 100%;
}
button {
    padding: 8px;
    background-color: var(--vibrant);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    width: 100%;
}
</style>
