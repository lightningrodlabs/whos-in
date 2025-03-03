<script lang="ts">
import Logo from "../../assets/whosin.svg";
import FaPlusCircle from 'svelte-icons/fa/FaPlusCircle.svelte';
import FaBell from 'svelte-icons/fa/FaBell.svelte';
import FaBullhorn from 'svelte-icons/fa/FaBullhorn.svelte';
import FaList from 'svelte-icons/fa/FaList.svelte';
import FaHome from 'svelte-icons/fa/FaHome.svelte';
import { navigate, view, weClientStored } from '../../store.js';
import Notifications from './Notifications.svelte';
import { clientContext } from '../../contexts';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
import { onMount, setContext, getContext } from 'svelte';
import { decode } from '@msgpack/msgpack';
import Avatar from "./Avatar.svelte";
import SvgIcon from '../../SvgIcon.svelte';
import { isWeContext } from "@lightningrodlabs/we-applet";
import AllNotifications from "./AllNotifications.svelte";
import SettingsModal from "./SettingsModal.svelte";
import { averageColor } from "../../crud/localStorage";

let headerColor = "#1952bb";
averageColor.subscribe(value => {
  headerColor = value?.rgba
  console.log("headerColor", headerColor);
  if (!headerColor) {
    headerColor = "#fff";
  }
});

let client: AppClient = (getContext(clientContext) as any).getClient();
let applets: Array<any> = (getContext(clientContext) as any).getApplets();
let weClient;
weClientStored.subscribe(value => {
  weClient = value;
});
let currentView;
let showSettingsModal = false;

view.subscribe(value => {
currentView = value;
});

async function goToCreate() {
navigate("create-coordination", {});
}

async function goToNotifications() {
navigate("notifications", {});
}

async function goToDashboard() {
navigate("dashboard", {});
}

async function goToBulletin() {
navigate("all-coordinations", {});
}

async function goToCalendar() {
navigate("calendar", {});
}
</script>

<header>
  <nav 
    class="navbar"
    style="background: {headerColor};"
  >
    <div class="container-fluid converge-header">
      <div>
        <!-- {#if !isWeContext()}
        <a id="logo" class="navbar-brand" on:click={() => navigate("instructions")}>
          <img class="logo-image" src={Logo} alt="whos-in logo"/>
        </a>
        {:else} -->
        <a id="logo" class="navbar-brand" on:click={() => navigate("instructions")}>  
          <h1 id="whosin-title" style="display: flex; align-items: center;">
            <img id="minilogo" src={Logo} alt="whos-in logo"/>
            <span>
              Who's In?
            </span>
          </h1>
        <!-- <small id="subtitle">for Moss</small> -->
        </a>
        
        <!-- {/if} -->
      </div>
    <div>

    <ul class="nav navbar-nav float-right">

    <li class="bulletin" on:click={goToBulletin}>
      {#if currentView == "all-coordinations"}
      <div class="bulletin-icon" style="color:#1952bb">
        <!-- <FaBullhorn />  -->
        <SvgIcon icon="faBullhorn" color=#1952bb />
        <span>
          Bulletin
        </span>
      </div>
      {:else}
      <div class="bulletin-icon">
        <!-- <FaBullhorn />  -->
        <SvgIcon icon="faBullhorn" color=#d6ddeb />
        <span>
          Bulletin
        </span>
      </div>
      {/if}
    </li>

    <!-- <li class="dashboard" on:click={goToDashboard}>
      {#if currentView == "dashboard"}
      <div class="dashboard-icon" style="color:#1952bb">
        <SvgIcon icon="faList" color=#1952bb />
        <span>
          Joined
        </span>
      </div>
      {:else}
      <div class="dashboard-icon">
        <SvgIcon icon="faList" color=#d6ddeb />
        <span>
          Joined
        </span>
      </div>
      {/if}
    </li> -->

    <li class="calendar" on:click={goToCalendar}>
      {#if currentView == "calendar"}
      <div class="dashboard-icon" style="color:#1952bb">
        <SvgIcon icon="faCalendar" size=18 color=#1952bb /> 
        <span>
          Calendar
        </span>
      </div>
      {:else}
      <div class="dashboard-icon">
        <SvgIcon icon="faCalendar" size=18 color=#d6ddeb />
        <span>
          Calendar
        </span>
      </div>
      {/if}
    </li>

    <li class="notifications-li">
      <div class="dropdown">
        <div class="notifications" on:click={goToNotifications} on:mouseover={() => document.getElementById('notifications-dropdown').style.display = 'block'} on:mouseleave={() => document.getElementById('notifications-dropdown').style.display = 'none'}>
          {#if currentView == "notifications"}
            <SvgIcon icon="faBell" color=#1952bb /> Notifications
          {:else}
            <SvgIcon icon="faBell" color=#d6ddeb /> Notifications
          {/if}
          <span class="notifications-count">
            <Notifications client={client}></Notifications>
          </span>
        </div>
        <div id="notifications-dropdown" class="dropdown-content" on:mouseover={() => document.getElementById('notifications-dropdown').style.display = 'block'} on:mouseleave={() => document.getElementById('notifications-dropdown').style.display = 'none'}>
          <!-- Add your notification items here -->
           <AllNotifications client={client}></AllNotifications>
          <!-- <div>No new notifications</div> -->
           <!-- go to all notificaitons button -->
          <div on:click={goToNotifications}>See all notifications</div>
        </div>
      </div>
    </li>
  
    {#if !applets}
    <svg xmlns="http://www.w3.org/2000/svg" style="margin: 0 10" width="1" height="30" viewBox="0 0 1 30"><defs><style>.a{fill:none;stroke:rgba(0,0,0,0.15);}</style></defs><line class="a" y2="30" transform="translate(0.5)"/></svg>

    <li class="middle-of-header-right">
      <div class="dropdown">
        <div class="new-action-button" on:mouseover={() => document.getElementById('dropdown-content').style.display = 'block'} on:mouseleave={() => document.getElementById('dropdown-content').style.display = 'none'}>
          <div class="icon">
            <FaPlusCircle />
          </div>
          <span id="new-action">Create</span>
        </div>
        <div id="dropdown-content" class="dropdown-content" on:mouseover={() => document.getElementById('dropdown-content').style.display = 'block'} on:mouseleave={() => document.getElementById('dropdown-content').style.display = 'none'}>
          <div on:click={() => navigate("create-event")}>Event</div>
          <div on:click={() => navigate("create-agreement")}>Agreement</div>
          <!-- <div on:click={() => navigate("create-project")}>Project</div> -->
        </div>
      </div>
    </li>

    <svg xmlns="http://www.w3.org/2000/svg" style="margin: 0 10" width="1" height="30" viewBox="0 0 1 30"><defs><style>.a{fill:none;stroke:rgba(0,0,0,0.15);}</style></defs><line class="a" y2="30" transform="translate(0.5)"/></svg>
    <li class="notifications-li">
      <div class="dropdown"></div>
      <div class="avatar">
      <Avatar showNickname={true} agentPubKey={client.myPubKey} size={24} namePosition="row"></Avatar>
      </div>
    </li>

    <li class="settings-li">
      <div class="settings" on:click={() => {showSettingsModal = !showSettingsModal}}>
      <SvgIcon icon="faCog" size={24} color="#d6ddeb" />
      </div>
      {#if showSettingsModal}
      <div id="settings-dropdown" class="dropdown-content">
        <SettingsModal client={client}></SettingsModal>
      </div>
      {/if}
    </li>
    {/if}
    <!-- if no agent linked to my agent as notifier -->
    <!-- {#if !notifier}
      <svg xmlns="http://www.w3.org/2000/svg" style="margin: 0 10" width="1" height="30" viewBox="0 0 1 30"><defs><style>.a{fill:none;stroke:rgba(0,0,0,0.15);}</style></defs><line class="a" y2="30" transform="translate(0.5)"/></svg>
      <button class="btn btn-primary" on:click={() => navigate("create-profile")}>Create Profile</button>
    {/if} -->
    
    <!-- <svg xmlns="http://www.w3.org/2000/svg" style="margin: 0 10" width="1" height="30" viewBox="0 0 1 30"><defs><style>.a{fill:none;stroke:rgba(0,0,0,0.15);}</style></defs><line class="a" y2="30" transform="translate(0.5)"/></svg> -->

    <!-- <profiles-context>
      <agent-avatar>
    </profiles-context> -->
    <!-- <my-profile></my-profile> -->
    <!-- <profile-detail agentPubKey={client.myPubKey}></profile-detail> -->
    <!-- <img class="nav-image" src="/assets/Line11-9bb361353b5c1a0dcd5b57d071c00a3edc6d82426e0c26bf0145a6dfd1e90081.svg" alt="Line11" border="0"> -->

    <!-- <li class="user-nav-link">
      <a href="/users/b9YtZb">
        <img alt="admin" class="gravatar" src="https://secure.gravatar.com/avatar/357f7f405fad3b4e96ff277c3c05a4de?s=80&amp;d=identicon">
        <span>admin</span>
      </a>        
    </li> -->

    <!-- <svg xmlns="http://www.w3.org/2000/svg" style="margin: 0 10" width="1" height="30" viewBox="0 0 1 30"><defs><style>.a{fill:none;stroke:rgba(0,0,0,0.15);}</style></defs><line class="a" y2="30" transform="translate(0.5)"/></svg>
    <li class="notifications-li">
      <Avatar showNickname={true} agentPubKey={client.myPubKey}  size={24} namePosition="row"></Avatar>
    </li> -->
    
    </ul>
    </div><!-- /.navbar-collapse -->
    </div><!-- /.container-fluid -->
  </nav>
</header>

<style>
  .converge-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  #whosin-title {
    /* font-size: 24px;
    font-weight: 600;
    color: #1952bb;
    margin: 0;
    font-family: 'Montserrat', sans-serif;
    letter-spacing: 3.15px;
    font-style: italic; */

    font-size: 24px;
    font-weight: 600;
    color: #1952bb;
    margin: 0;
    font-family: Montserrat, sans-serif;
    letter-spacing: 3.15px;
    padding: 4px 10px;

  }
  #subtitle {
    font-size: 12px;
    font-weight: 600;
    color: #3fadab;
    margin: 0;
    letter-spacing: 1.15px;
  }

  #minilogo {
    width: 1.7em;
    margin: 1px 6px 0 -4px;
    padding: 1px;
    box-shadow: none;
  }

  #logo {
    cursor: pointer;
  }

  .navbar-nav > li > div {
    display: flex;
    align-items: center;
    cursor: pointer;
  }

  .navbar-nav > li > div > span {
    margin-left: 4px;
  }

  .dropdown {
    position: relative;
    display: inline-block;
  }

  .dropdown-content {
    display: none;
    position: absolute;
    background-color: #f9f9f9;
    min-width: 160px;
    box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
    z-index: 1;
    top: 100%; /* Position the dropdown below the button */
    right: 0; /* Align the dropdown to the right */
  }

  .dropdown-content div {
    color: black;
    padding: 12px 16px;
    text-decoration: none;
    display: block;
  }

  .dropdown-content div:hover {
    background-color: #f1f1f1;
  }

  #notifications-dropdown {
    width: 300px;
  }
  
  .notifications-li {
    margin-right: 10px;
  }

  :global(body.dark-mode) #whosin-title {
    color: white;
  }

  :global(body.dark-mode) .new-action-button {
    background: #859dca;
  }
</style>