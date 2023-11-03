<script lang="ts">
  import { createEventDispatcher, onMount, setContext } from 'svelte';
  import type { ActionHash, AgentPubKey, AppAgentClient, AppSignalCb } from '@holochain/client';
  import { AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import { view, viewHash, navigate } from './store.js';
  import { clientContext } from './contexts';
  import Header from './whosin/coordinator/Header.svelte';
  import CreateCoordination from './whosin/coordinator/CreateCoordination.svelte';
  import AllCoordinations from './whosin/coordinator/AllCoordinations.svelte';
  import CoordinationDetail from './whosin/coordinator/CoordinationDetail.svelte';
  import AllNotifications from './whosin/coordinator/AllNotifications.svelte';
  import Instructions from './whosin/coordinator/Instructions.svelte';
  import MyCoordinations from './whosin/coordinator/MyCoordinations.svelte';
  import { WeClient, isWeContext } from '@lightningrodlabs/we-applet';
  import CreateTwilioCredentials from './whosin/notifications/CreateTwilioCredentials.svelte';
  import CreateContact from './whosin/notifications/CreateContact.svelte';
  import NotificationsHandler from './whosin/notifications/NotificationsHandler.svelte';
  import Holochain from "./assets/holochain.png";

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
  
  const dispatch = createEventDispatcher();

  let client: AppAgentClient | undefined;
  let loading = true;
  let store = undefined;
  let currentView: String;
  let currentHash: Uint8Array;
  let notifier: AgentPubKey | undefined;
  let dna;

  $: client, loading, store, notifier, dna;

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

  async function checkIfNew() {
      try {
          const records = await client
          .callZome({
              cap_secret: null,
              role_name: 'whosin',
              zome_name: 'coordinator',
              fn_name: 'get_my_coordination_hashes',
              payload: null,
          });

          if (records.length > 0) {
              navigate('dashboard');
          } else {
              navigate('');
          }

      } catch (e) {
          console.log(e)
      }
      loading = false;
  }

  async function checkForNotifier() {
    try {
        const record = await client
        .callZome({
            cap_secret: null,
            role_name: 'whosin',
            zome_name: 'notifications',
            fn_name: 'get_my_notifier',
            payload: null,
        });
        notifier = record;
    } catch (e) {
        // console.log(e)
        let error = e;
    }
  }

  onMount(async () => {
    if (isWeContext()) {
      const weClient = await WeClient.connect();
      console.log(weClient.renderInfo)

      if (
        !(weClient.renderInfo.type === "applet-view")
        && !(weClient.renderInfo.view.type === "main")
      ) throw new Error("This Applet only implements the applet main view.");

      client = weClient.renderInfo.appletClient;
      // const profilesClient = weClient.renderInfo.profilesClient;
    } else {
      // We pass '' as url because it will dynamically be replaced in launcher environments
      client = await AppAgentWebsocket.connect('', 'whosin');
    }
    console.log("client... ", client.appInfo())


    await checkForNotifier();

    if (currentView == "home") {
      checkIfNew()
    }
    try {
      dna = await client
        .callZome({
            cap_secret: null,
            role_name: 'whosin',
            zome_name: 'coordinator',
            fn_name: 'get_dna_hash',
            payload: null,
        });
        console.log("dna")
      console.log(dna)
    } catch (e) {
      console.log("no dna")

      console.log(e)
    }

    // const config = {
      // avatarMode: "identicon",
      // additionalFields: ["Location", "Bio"], // Custom app level profile fields
    // };
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

  async function alert_ui() {
    try {
        const record = await client
        .callZome({
            cap_secret: null,
            role_name: 'whosin',
            zome_name: 'notifications',
            fn_name: 'notification_tip',
            payload: "client.myPubKey",
        });
        notifier = record;
    } catch (e) {
        console.log(e.data.data)
    }
	}

</script>

{#if client}
<NotificationsHandler></NotificationsHandler>
<main>
  <!-- <button on:click={() => alert_ui()}>alert ui</button> -->
  <!-- <TwilioCredentialsDetail></TwilioCredentialsDetail> -->
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

    <!-- <CreateContact></CreateContact> -->
    <!-- <ContactDetail></ContactDetail> -->

    
    {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate />
    </div>
    {:else if currentView == "coordination"}
    <CoordinationDetail coordinationHash={currentHash}></CoordinationDetail>
    {:else if currentView == "create-coordination"}
    <CreateCoordination></CreateCoordination>
    <!-- HI -->
    {:else if currentView == "notifications"}
      {#if true && !loading && !notifier && !(["notifier", "notificant", "home", "create-coordination"].includes(String(currentView)))}
        <button on:click={() => navigate('notificant')}>Sign up for text notifications</button>
      {/if}
      <AllNotifications></AllNotifications>
      {:else if currentView == "dashboard"}
      <MyCoordinations></MyCoordinations>
    {:else if currentView == "all-coordinations"}
      <AllCoordinations></AllCoordinations>
    {:else if currentView == "notifier"}
      <CreateTwilioCredentials></CreateTwilioCredentials>
    {:else if currentView == "notificant"}
      <CreateContact></CreateContact>
    {:else}
      <!-- <CreateCoordination></CreateCoordination> -->
      <!-- <profile-detail /> -->

      <Instructions></Instructions>
    {/if}
  <!-- </profile-prompt> -->
  <!-- </profiles-context> -->
  <br>
  <small>
    <img class="holochain-logo" src={Holochain} alt="holochain logo"/>
    Private Holochain network: {dna}</small>
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

  .holochain-logo {
    height: 1em;
    top: 0.1em;
    position: relative;
  }
</style>
{/if}
