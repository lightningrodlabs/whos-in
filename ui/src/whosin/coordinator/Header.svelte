<script lang="ts">
    import Logo from "../../assets/logo.png";
    import FaPlusCircle from 'svelte-icons/fa/FaPlusCircle.svelte';
    import FaBell from 'svelte-icons/fa/FaBell.svelte';
    import FaBullhorn from 'svelte-icons/fa/FaBullhorn.svelte';
    import FaList from 'svelte-icons/fa/FaList.svelte';
    import FaHome from 'svelte-icons/fa/FaHome.svelte';
    import { navigate } from '../../store.js';
    import Notifications from './Notifications.svelte';
    import { clientContext } from '../../contexts';
    import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
    import { onMount, setContext, getContext } from 'svelte';
    import { decode } from '@msgpack/msgpack';
    
    // import {
    //   ProfilesStore,
    //   ProfilesClient,
    //   CreateProfile,
    //   ProfilePrompt,
    //   profilesStoreContext,
    //   MyProfile,
    //   ProfilesContext,
    //   ProfileDetail,
    //   ProfileListItemSkeleton,
    //   AgentAvatar,
    // } from '@holochain-open-dev/profiles';
    let client: AppAgentClient = (getContext(clientContext) as any).getClient();
    
    // let client: AppAgentClient;
    // let myAgentPubKey: AgentPubKey = (getContext(clientContext) as any).getClient();
    
    // if (!customElements.get('my-profile')){
    //   customElements.define('my-profile', MyProfile)
    // }

    // if (!customElements.get('profile-detail')){
    //   customElements.define('profile-detail', ProfileDetail)
    // }

    // if (!customElements.get('profile-list-item-skeleton')){
    //   customElements.define('profile-list-item-skeleton', ProfileListItemSkeleton)
    // }

    // if (!customElements.get('agent-avatar')){
    //   customElements.define('agent-avatar', AgentAvatar)
    // }

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
    
    <!-- <div>{JSON.stringify(myAgentPubKey["myPubKey"])}</div> -->

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
              <FaBullhorn />
            </div>
          </li>
    
          <li class="dashboard" on:click={goToDashboard}>
            <div class="dashboard-icon">
              <FaList />
            </div>
          </li>
    
          <li class="notifications-li">
            <div class="notifications" on:click={goToNotifications}>
              <FaBell />
              <span class="notifications-count">
                <!-- {#if client} -->
                <Notifications client={client}></Notifications>
                <!-- {/if} -->
              </span>
            </div>
          </li>
        
          <svg xmlns="http://www.w3.org/2000/svg" style="margin: 0 10" width="1" height="30" viewBox="0 0 1 30"><defs><style>.a{fill:none;stroke:rgba(0,0,0,0.15);}</style></defs><line class="a" y2="30" transform="translate(0.5)"/></svg>

          <li class="middle-of-header-right"> 
            <div class="new-action-button"  on:click={goToCreate}>
              <div class="icon">
                <FaPlusCircle />
              </div>
              <!-- <i class="fas fa-plus white-circle-plus"></i> -->
              <!-- <img class="nav-image" src="/assets/add_circle_black_24dp-b42cee553b2665d6f62bd5d9ffc02837cf3c5a3084fc6a5674f5edf83776f565.svg" alt="Add circle black 24dp" border="0"> -->
              <span id="new-action">New action</span>
            </div>
          </li>
    
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
    
          </ul>
          </div><!-- /.navbar-collapse -->
          </div><!-- /.container-fluid -->
        </nav>
      </header>
    