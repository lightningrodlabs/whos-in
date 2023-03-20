<script lang="ts">
    import Logo from "../../assets/logo.png";
    // import FaPlusCircle from 'svelte-icons/fa/FaPlusCircle.svelte';
    // import FaBell from 'svelte-icons/fa/FaBell.svelte';
    // import FaHome from 'svelte-icons/fa/FaHome.svelte';
    // import FaList from 'svelte-icons/fa/FaList.svelte';
    import { navigate } from '../../store.js';
    import Notifications from './Notifications.svelte';
    import { clientContext } from '../../contexts';
    import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
    import { onMount, setContext, getContext } from 'svelte';
    
    // let client: AppAgentClient = (getContext(clientContext) as any).getClient();
    
    export let client: AppAgentClient;
    
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
    </script>
    
    <header>
        <nav class="navbar">
          <div class="container-fluid converge-header">
            <div>
              <a id="logo" class="navbar-brand" href="/">
                <img class="logo-image" src={Logo} alt="whos-in logo"/>
              </a>    
            </div>
          <div>
    
          <ul class="nav navbar-nav float-right">
    
          <li class="bulletin" on:click={goToBulletin}>
            <div class="bulletin-icon">
              <!-- <FaHome /> -->
              H
            </div>
          </li>
    
          <li class="dashboard" on:click={goToDashboard}>
            <div class="dashboard-icon">
              <!-- <FaList /> -->
              D
            </div>
          </li>
    
          <li class="notifications-li">
            <div class="notifications" on:click={goToNotifications}>
              <!-- <FaBell /> -->
              N
              <span class="notifications-count">
                <!-- {#if client} -->
                <Notifications client={client}></Notifications>
                <!-- {/if} -->
              </span>
            </div>
          </li>
        
          <!-- <img class="nav-image"> -->
    
          <li>
            <div class="new-action-button"  on:click={goToCreate}>
              <div class="icon">
                <!-- <FaPlusCircle /> -->
                +
              </div>
              <!-- <i class="fas fa-plus white-circle-plus"></i> -->
              <!-- <img class="nav-image" src="/assets/add_circle_black_24dp-b42cee553b2665d6f62bd5d9ffc02837cf3c5a3084fc6a5674f5edf83776f565.svg" alt="Add circle black 24dp" border="0"> -->
              <span id="new-action">New action</span>
            </div>     
          </li>
    
          <!-- <img class="nav-image" src="/assets/Line11-9bb361353b5c1a0dcd5b57d071c00a3edc6d82426e0c26bf0145a6dfd1e90081.svg" alt="Line11" border="0"> -->
    
          <!-- <li class="user-nav-link">
            <a href="/users/b9YtZb">
              <img alt="admin" class="gravatar" src="https://secure.gravatar.com/avatar/357f7f405fad3b4e96ff277c3c05a4de?s=80&amp;d=identicon">
              <span>admin</span>
            </a>        
          </li> -->
    
          </ul>
          </div><!-- /.navbar-collapse -->
          </div><!-- /.container-fluid -->
        </nav>
      </header>
    