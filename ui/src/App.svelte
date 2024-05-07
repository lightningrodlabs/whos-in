<script lang="ts">
  import { createEventDispatcher, onMount, setContext } from 'svelte';
  import type { ActionHash, AgentPubKey, AppAgentClient, AppSignalCb } from '@holochain/client';
  import { AppAgentWebsocket, AdminWebsocket } from '@holochain/client';
  import '@shoelace-style/shoelace/dist/themes/light.css';
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-prompt.js";
  import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
  import "@holochain-open-dev/profiles/dist/elements/list-profiles.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-list-item-skeleton.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-detail.js";
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import '@material/mwc-circular-progress';
  import { view, viewHash, navigate, setWeClient } from './store.js';
  import { clientContext, profilesStoreContext } from './contexts';
  import { ProfilesStore, ProfilesClient } from "@holochain-open-dev/profiles";
  import Header from './whosin/coordinator/Header.svelte';
  import CreateCoordination from './whosin/coordinator/CreateCoordination.svelte';
  import AllCoordinations from './whosin/coordinator/AllCoordinations.svelte';
  import CoordinationDetail from './whosin/coordinator/CoordinationDetail.svelte';
  import AllNotifications from './whosin/coordinator/AllNotifications.svelte';
  import Instructions from './whosin/coordinator/Instructions.svelte';
  import MyCoordinations from './whosin/coordinator/MyCoordinations.svelte';
  import CreateTwilioCredentials from './whosin/notifications/CreateTwilioCredentials.svelte';
  import CreateContact from './whosin/notifications/CreateContact.svelte';
  import NotificationsHandler from './whosin/notifications/NotificationsHandler.svelte';
  import Holochain from "./assets/holochain.png";
  import { WeClient, isWeContext, initializeHotReload } from '@lightningrodlabs/we-applet';  
  import { appletServices } from './we';
  import SvgIcon from './SvgIcon.svelte';
  import AllViewed from './whosin/coordinator/AllViewed.svelte';
  import { fade } from 'svelte/transition'
  
  const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'converge'
  const roleName = 'converge'
  const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
  const adminPort = import.meta.env.VITE_ADMIN_PORT
  const url = `ws://localhost:${appPort}`;

  const dispatch = createEventDispatcher();

  let client: AppAgentClient | undefined;
  let loading = true;
  let store = undefined;
  let currentView: String;
  let currentHash: Uint8Array;
  let notifier: AgentPubKey | undefined;
  let allNotifiers: Array<AgentPubKey> | undefined;
  let dna;
  let profilesStore = undefined;
  let connected = false
  let weClient: WeClient
  $: client, loading, store, notifier, dna;

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
        const record2 = await client
        .callZome({
          cap_secret: null,
          role_name: 'whosin',
          zome_name: 'notifications',
          fn_name: 'list_notifiers',
          payload: null,
        });
        console.log("all notifiers", record2)
        allNotifiers = record2;

        const record = await client
        .callZome({
            cap_secret: null,
            role_name: 'whosin',
            zome_name: 'notifications',
            fn_name: 'get_my_notifier',
            payload: null,
        });
        console.log("notifier", record)
        notifier = record;

      } catch (e) {
        // console.log(e)
        let error = e;
    }
  }


  async function initialize() : Promise<void> {
    console.log(import.meta.env)
    let profilesClient
    if ((import.meta as any).env.DEV) {
      try {
        await initializeHotReload();
      } catch (e) {
        console.warn("Could not initialize applet hot-reloading. This is only expected to work in a We context in dev mode.")
      }
    }
    if (!isWeContext()) {
      console.log("adminPort is", adminPort)
      if (adminPort) {
        const url = `ws://localhost:${adminPort}`
        console.log("connecting to admin port at:", url)
        const adminWebsocket = await AdminWebsocket.connect({url: new URL(url)})
        const x = await adminWebsocket.listApps({})
        console.log("apps", x)
        const cellIds = await adminWebsocket.listCellIds()
        console.log("CELL IDS",cellIds)
        await adminWebsocket.authorizeSigningCredentials(cellIds[0])
      }
      console.log("appPort and Id is", appPort, appId)
      client = await AppAgentWebsocket.connect(appId,{url: new URL(url)})
      profilesClient = new ProfilesClient(client, appId);
    
      // client = await AppAgentWebsocket.connect('', 'dcan');
      // profilesStore = new ProfilesStore(new ProfilesClient(client, 'converge'), {
      //   avatarMode: "avatar-optional",
      //   minNicknameLength: 3,
      // });
    }
    else {
      // const weClient = await WeClient.connect();
      weClient = await WeClient.connect(appletServices);
      // store set
      setWeClient(weClient)
      // weClient = await WeClient.connect();

      switch (weClient.renderInfo.type) {
        case "applet-view":
          switch (weClient.renderInfo.view.type) {
            case "main":
              // here comes your rendering logic for the main view
              break;
            case "block":
              switch(weClient.renderInfo.view.block) {
                case "active_boards":
                  currentView = "dashboard"
                  break;
                default:
                  throw new Error("Unknown applet-view block type:"+weClient.renderInfo.view.block);
              }
              break;
            case "creatable":
              switch (weClient.renderInfo.view.name) {
                case "Coordination":
                currentView = "create-coordination-mini"
              }
              break;
            case "asset":
              switch (weClient.renderInfo.view.roleName) {
                case "whosin":
                  switch (weClient.renderInfo.view.integrityZomeName) {
                    case "coordinator_integrity":
                      switch (weClient.renderInfo.view.entryType) {
                        case "coordination":
                          currentView = "coordination"
                          currentHash = weClient.renderInfo.view.wal.hrl[1]
                          // console.log("weClient.renderInfo.view", weClient.renderInfo.view)
                          // hrlWithContext = weClient.renderInfo.view.hrlWithContext
                          break;
                        default:
                          throw new Error("Unknown entry type:"+weClient.renderInfo.view.entryType);
                      }
                      break;
                    default:
                      throw new Error("Unknown integrity zome:"+weClient.renderInfo.view.integrityZomeName);
                  }
                  break;
                default:
                  throw new Error("Unknown role name:"+weClient.renderInfo.view.roleName);
              }
              break;
            default:
              throw new Error("Unsupported applet-view type");
          }
          break;
        case "cross-applet-view":
          switch (this.weClient.renderInfo.view.type) {
            case "main":
              // here comes your rendering logic for the cross-applet main view
              //break;
            case "block":
              //
              //break;
            default:
              throw new Error("Unknown cross-applet-view render type.")
          }
          break;
        default:
          throw new Error("Unknown render view type");

      }
      
      //@ts-ignore
      client = weClient.renderInfo.appletClient;
      //@ts-ignore
      profilesClient = weClient.renderInfo.profilesClient;
    }
    profilesStore = new ProfilesStore(profilesClient);
    connected = true
  }

  onMount(async () => {
    await initialize()
    await checkForNotifier()
    // client.on(
    //   'signal', 
    //   (signal) => {
    //     console.log(signal)
    //     if (signal.data.payload.signal_type == "notification") {
    //       console.log(signal.data.payload)
    //     }
    //   } 
    // );

    // await checkForNotifier();

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
        // console.log("dna")
      // console.log(dna)
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

  setContext(profilesStoreContext, {
    getProfileStore: () => profilesStore,
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
{#if profilesStore}
  <profiles-context store="{profilesStore}">
    <profile-prompt>
      {#if weClient.renderInfo.view.type != "attachable"}

      <NotificationsHandler></NotificationsHandler>
      <main style="width: 100vw;">

          {#if currentView != "create-coordination-mini"}
            <Header></Header>
          {/if}

          {#if !loading && !notifier && allNotifiers.length > 1 && !(["notifier", "notificant", "home", "create-coordination"].includes(String(currentView)))}
            <p class="notice" style="margin: auto; border-radius: 0 0 4px 4px">Want to receive texts or emails when coordinations reach minimum participation?
              <button on:click={() => navigate('notificant')}>Click here</button>
              <!-- {#if String(currentView) != "notifications"}
                <button>Dismiss</button>
              {/if} -->
            </p>
          {/if}
          
          {#if loading}
          <div style="display: flex; flex: 1; align-items: center; justify-content: center">
            <mwc-circular-progress indeterminate />
          </div>
          {:else if currentView == "coordination"}
          <div class="white-container" style="display: flex; flex-direction: column; margin-top: 30px;" in:fade={{duration: 200}} out:fade={{duration: 100}}>
          <CoordinationDetail coordinationHash={currentHash}></CoordinationDetail>
          </div>
          {:else if currentView == "create-coordination-mini"}
            <div style="padding: 10px;">
              <CreateCoordination></CreateCoordination>
            </div>
          {:else if currentView == "create-coordination"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <div class="white-container" style="display: flex; flex-direction: column; margin-top: 30px;">
                <CreateCoordination></CreateCoordination>
              </div>
            </span>
          <!-- HI -->
          {:else if currentView == "notifications"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <AllNotifications></AllNotifications>
            </span>
          {:else if currentView == "dashboard"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <MyCoordinations></MyCoordinations>
            </span>
          {:else if currentView == "all-coordinations"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <AllCoordinations></AllCoordinations>
            </span>
          {:else if currentView == "notifier"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <CreateTwilioCredentials></CreateTwilioCredentials>
            </span>
          {:else if currentView == "notificant"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <CreateContact></CreateContact>
            </span>
          {:else}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <Instructions></Instructions>
            </span>
          {/if}

        <footer style="margin: 10px;">
          <!-- feedback button -->
          <SvgIcon icon="faBug" size="24" color="#000000" />
          <a href="https://docs.google.com/forms/d/e/1FAIpQLSdzwS5D1HP3Eq6JV2lSD2cTXZoVTJJR2b7vEuAKgk9izVFRIw/viewform" target="_blank" class="feedback-button">
            <span>Submit feedback</span>
          </a>
          :)
        {#if !isWeContext && dna && !loading && currentView != "instructions" && currentView != "" && (!weClient || weClient.renderInfo.view.type != "attachable")}
        <small>
          <img class="holochain-logo" src={Holochain} alt="holochain logo"/>
          Private Holochain network: {dna}
        </small>
        {/if}
        </footer>
        <!-- </profile-prompt> -->
        <!-- </profiles-context> -->
        {#if false && dna && !loading && currentView != "instructions" && currentView != ""}
        <footer style="margin: 10px;">
        <small>
          <img class="holochain-logo" src={Holochain} alt="holochain logo"/>
          Private Holochain network: {dna}
        </small>
        </footer>
        {/if}

        <AllViewed />
      </main>
      {:else}
      <div class="attachment-container" style="display: flex; flex-direction: column">
      <CoordinationDetail coordinationHash={currentHash}></CoordinationDetail>
      </div>
      {/if}
    </profile-prompt>
  </profiles-context>
{/if}

<style>
  main {
    text-align: center;
    /* padding: 1em; */
    /* max-width: 240px; */
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }

  .holochain-logo {
    height: 1.2em;
    top: 0.3em;
    position: relative;
  }
</style>
{/if}
