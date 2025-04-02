<script lang="ts">
export let allUsers = [
    'Alice',
    'Bob',
    'Charlie',
    'David',
    'Eve',
    'Frank',
    'Grace',
    'Heidi',
    'Ivan',
    'Judy',
    'Kevin',
    'Laura',
    'Michael',
    'Nia',
    'Oscar',
    'Pam',
    'Quincy',
    'Rita',
    'Steve',
    'Tina',
    'Uma',
    'Victor',
    'Wendy',
    'Xavier',
    'Yvonne',
    'Zack'
];
export let selectedUsers = [...allUsers];
let displayedUsers = [...allUsers];

</script>

<h1>Find a time</h1>

<!-- form that provides a list of users and lets you check/uncheck and search-filter them -->
<form>
    <!-- input to determine a period of time period in 15-minute increments -->
     <div class="form-group"
        style="flex-direction: column; align-items: center;"
     >
         <label for="time">Select a duration:</label>
         <!-- default to 1hr 15 minutes -->
        <div>
            <input type="number" id="hours" name="hours" min="0" max="24" value="1" /> hours
            <input type="number" id="minutes" name="minutes" min="0" max="59" value="15" /> minutes
        </div>
    </div>

    <div class="form-group">
        <input type="text" placeholder="Search for users..." 
        style="margin-right: 1em;"
            on:input={(e) => {
            displayedUsers = allUsers.filter(u => u.toLowerCase().includes(e.target.value.toLowerCase()));
            }}
        />
        <!-- unselect/select all users -->
        <button type="button" on:click={() => {
            if (selectedUsers.length === allUsers.length) {
            selectedUsers = [];
            } else {
            selectedUsers = [...allUsers];
            }
        }}>
            {selectedUsers.length === allUsers.length ? 'unselect all' : 'select all'}
        </button>
    </div>
  <div class="user-list">
    {#each displayedUsers as user}
      <div class="form-group-checkbox">
        <input type="checkbox" id={user} name={user} value={user} checked={selectedUsers.includes(user)}
            on:change={() => {
                if (selectedUsers.includes(user)) {
                selectedUsers = selectedUsers.filter(u => u !== user);
                } else {
                selectedUsers = [...selectedUsers, user];
                }
            }}
        />
        <label for={user}>{user}</label>
      </div>
    {/each}
  </div>

  <!-- apply -->
    <button type="submit" 
    style="margin-top: 1em;"
    on:click={(e) => {
        e.preventDefault();
        console.log(selectedUsers);
    }}>Apply</button>
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
    margin-bottom: 8px;
}
label {
    margin-left: 8px;
}
input[type="text"] {
    width: 100%;
    padding: 8px;
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
