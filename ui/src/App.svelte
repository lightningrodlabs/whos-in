<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import { view, viewHash, navigate } from './store.js';
  import { clientContext } from './contexts';
  import Header from './whosin/coordinator/Header.svelte';
  import CreateCoordination from './whosin/coordinator/CreateCoordination.svelte';
  import AllCoordinations from './whosin/coordinator/AllCoordinations.svelte';
  import CoordinationDetail from './whosin/coordinator/CoordinationDetail.svelte';
  import AllNotifications from './whosin/coordinator/AllNotifications.svelte';
  import MyCoordinations from './whosin/coordinator/MyCoordinations.svelte';

  // import {
  //   ProfilesStore,
  //   ProfilesClient,
  //   CreateProfile,
  //   ProfilePrompt,
  //   profilesStoreContext,
  //   MyProfile,
  //   ProfilesContext,
  //   AgentAvatar,
  //   ProfileDetail,
  //   ListProfiles,
  // } from '@holochain-open-dev/profiles';
  
  let client: AppAgentClient | undefined;
  let loading = true;
  let store = undefined;
  let currentView: String;
  let currentHash: Uint8Array;

  $: client, loading, store;

  // if (!customElements.get('profiles-context')){
  //   customElements.define('profiles-context', ProfilesContext)
  // }

  // if (!customElements.get('my-profile')){
  //   customElements.define('my-profile', MyProfile)
  // }

  // if (!customElements.get('profile-prompt')){
  //   customElements.define('profile-prompt', ProfilePrompt)
  // }

  // if (!customElements.get('agent-avatar')){
  //   customElements.define('agent-avatar', AgentAvatar)
  // }

  // if (!customElements.get('agent-avatar')){
  //   customElements.define('agent-avatar', AgentAvatar)
  // }
  
  // if (!customElements.get('profile-detail')){
  //   customElements.define('profile-detail', ProfileDetail)
  // }

  // if (!customElements.get('list-profiles')){
  //   customElements.define('list-profiles', ListProfiles)
  // }

  onMount(async () => {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect('', 'dcan');
    const config = {
      // avatarMode: "identicon",
      // additionalFields: ["Location", "Bio"], // Custom app level profile fields
    };
    // store = new ProfilesStore(new ProfilesClient(client, 'whosin'), config);
    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });

  view.subscribe(value => {
    currentView = value;
  });

  viewHash.subscribe(value => {
    currentHash = value;
  });
</script>

{#if client}
<!-- {client.myPubKey} -->
<main>
  <!-- <profiles-context store={store}> -->
    <!-- <agent-avatar /> -->

    <!-- <profile-detail agentPubKey={client.myPubKey}> -->
  <!-- </profile-detail> -->
    <!-- </agent-avatar> -->
  <!-- </profiles-context> -->
  <!-- <profiles-context store={store}> -->
    <!-- <agent-avatar></agent-avatar> -->

    <!-- <profile-detail /> -->
    <!-- <list-profiles /> -->
    <Header></Header>
    <!-- <profile-prompt> -->
      <!-- <profile-detail /> -->

    <!-- <agent-avatar></agent-avatar> -->
    <!-- <profile-details></profile-details> -->
      <!-- <my-profile></my-profile> -->
    <!-- <div style="margin: 20px;"></div> -->
    {#if loading}
      <div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate />
      </div>
    {:else if currentView == "coordination"}
      <div id="content"><CoordinationDetail coordinationHash={currentHash}></CoordinationDetail></div>
    {:else if currentView == "create-coordination"}
      <div id="content"><CreateCoordination></CreateCoordination></div>
      <!-- HI -->
    {:else if currentView == "notifications"}
      <div id="content"><AllNotifications></AllNotifications></div>
      {:else if currentView == "dashboard"}
      <div id="content"><MyCoordinations></MyCoordinations></div>
    {:else if currentView == "all-coordinations"}
      <div id="content"><AllCoordinations></AllCoordinations></div>
    {:else}
      <!-- <CreateCoordination></CreateCoordination> -->
      <!-- <profile-detail /> -->

      <AllCoordinations></AllCoordinations>
      
    {/if}
  <!-- </profile-prompt> -->
  <!-- </profiles-context> -->
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
{/if}
