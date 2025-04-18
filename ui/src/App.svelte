<script lang="ts">
  import { createEventDispatcher, onMount, setContext } from 'svelte';
  import type { ActionHash, AgentPubKey, AppClient, AppSignalCb, AppWebsocketConnectionOptions } from '@holochain/client';
  import { AppWebsocket, AdminWebsocket } from '@holochain/client';
  import '@shoelace-style/shoelace/dist/themes/light.css';
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-prompt.js";
  import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
  import "@holochain-open-dev/profiles/dist/elements/list-profiles.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-list-item-skeleton.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-detail.js";
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import '@material/mwc-circular-progress';
  import { view, viewHash, navigate, setWeaveClient } from './store.js';
  import { clientContext, profilesStoreContext } from './contexts';
  import { ProfilesStore, ProfilesClient } from "@holochain-open-dev/profiles";
  import Header from './whosin/coordinator/Header.svelte';
  import CreateCoordination from './whosin/coordinator/Creation/CreateCoordination.svelte';
  import AllCoordinations from './whosin/coordinator/AllCoordinations.svelte';
  import CoordinationDetail from './whosin/coordinator/CoordinationDetail.svelte';
  import AllNotifications from './whosin/coordinator/AllNotifications.svelte';
  import Instructions from './whosin/coordinator/Instructions.svelte';
  import MyCoordinations from './whosin/coordinator/MyCoordinations.svelte';
  import CreateTwilioCredentials from './whosin/notifications/CreateTwilioCredentials.svelte';
  import CreateContact from './whosin/notifications/CreateContact.svelte';
  import NotificationsHandler from './whosin/notifications/NotificationsHandler.svelte';
  import Holochain from "./assets/holochain.png";
  import { WeaveClient, isWeaveContext, initializeHotReload } from '@theweave/api';  
  import { appletServices } from './we';
  import SvgIcon from './SvgIcon.svelte';
  import AllViewed from './whosin/coordinator/AllViewed.svelte';
  import { fade } from 'svelte/transition'
  import { refetchCoordinations } from './crud/refetch.js';
  import app from './main.js';
  import Calendar from './whosin/coordinator/Calendaring/Calendar.svelte';
  import { averageColor, backgroundImage, loadState, setAverageColor, setColorPalette, colorPalette, setBackgroundImage } from './crud/localStorage.js';
  import { FastAverageColor } from 'fast-average-color';
  import Sync from './Sync.svelte';
  import { Vibrant } from "node-vibrant/browser";

  const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'whosin'
  const roleName = 'whosin'
  const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
  const adminPort = import.meta.env.VITE_ADMIN_PORT
  const url = `ws://localhost:${appPort}`;

  const dispatch = createEventDispatcher();

  let client: AppClient | undefined;
  let applets;
  let loading = true;
  let store = undefined;
  let currentView: string = "all-coordinations";
  let currentHash: Uint8Array;
  let notifier: AgentPubKey | undefined;
  let allNotifiers: Array<AgentPubKey> | undefined;
  let dna;
  let profilesStore = undefined;
  let connected = false
  let weClient: WeaveClient
  $: client, loading, store, notifier, dna;

  let defaultPalette = {
    Vibrant: { rgb: [25, 82 ,187] },
    Muted: { rgb: [50, 73, 115] },
    DarkMuted: { rgb: [27, 62, 127] },
    LightVibrant: { rgb: [195, 195, 195] },
    DarkVibrant: { rgb: [101, 120, 159] },
    LightMuted: { rgb: [177, 177, 177] },
    LightMutedTransparent: { rgb: [163, 163, 163] },
  }

  function setDefaultPalette() {
    document.body.style.background = "#e6ecf8";
    setBackgroundImage(null);
    setAverageColor({
      rgba: "rgb(255,255,255,1)",
      rgb: "rgb(255,255,255)",
      isDark: false,
    })
    setColorPalette(defaultPalette);
        // load default colors
    document.documentElement.style.setProperty("--vibrant", 'rgb(25,82,187)');
    document.documentElement.style.setProperty("--muted", 'rgb(50, 73, 115)');
    document.documentElement.style.setProperty("--dark-muted", 'rgb(27, 62, 127)');
    document.documentElement.style.setProperty("--light-vibrant", 'rgb(195, 195, 195)');
    document.documentElement.style.setProperty("--dark-vibrant", 'rgb(101,120,159)');
    document.documentElement.style.setProperty("--light-muted", 'rgb(177,177,177)');
    document.documentElement.style.setProperty("--light-muted-transparent", 'rgba(198,198,198,0.5)');

  }

  // $: document.body.style.background = "black";
  backgroundImage.subscribe(value => {
    if (!value || value == "none") {
      setDefaultPalette();
      return;
    } else {
      document.body.style.background = `url(${value}) no-repeat center center fixed`;
      document.body.style.backgroundSize = 'cover';
      // document.body.style.backdropFilter = 'brightness(30%)';
      const fac = new FastAverageColor();
      fac.getColorAsync(value)
      .then(color => {
        // container.style.backgroundColor = color.rgba;
        // container.style.color = color.isDark ? '#fff' : '#000';
        setAverageColor(color);
        console.log('Average color', color);
      })
      .catch(e => {
        console.log(e);
      });
      
      console.log("value palette", value);
      
      Vibrant.from(value)
      .getPalette()
      .then((palette) => {
        console.log("palette", palette);
        setColorPalette(palette);
        document.documentElement.style.setProperty("--vibrant", 'rgb(' + palette.Vibrant.rgb.join(",") + ')');
        document.documentElement.style.setProperty("--muted", 'rgb(' + palette.Muted.rgb.join(",") + ')');
        document.documentElement.style.setProperty("--dark-muted", 'rgb(' + palette.DarkMuted.rgb.join(",") + ')');
        document.documentElement.style.setProperty("--light-vibrant", 'rgb(' + palette.LightVibrant.rgb.join(",") + ')');
        document.documentElement.style.setProperty("--dark-vibrant", 'rgb(' + palette.DarkVibrant.rgb.join(",") + ')');
        document.documentElement.style.setProperty("--light-muted", 'rgb(' + palette.LightMuted.rgb.join(",") + ')');
        document.documentElement.style.setProperty("--light-muted-transparent", 'rgba(' + palette.LightMuted.rgb.join(",") + ',0.5)');
        console.log("palette", palette, 'rgb(' + palette.Vibrant.rgb.join(",") + ')');
      }).catch(() => {
        setDefaultPalette();
      })
    }
  });
    
  averageColor.subscribe(value => {
    if (!value) return;
    document.body.classList.toggle("dark-mode", value?.isDark);
  });

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
    loadState();

    console.log(import.meta.env)
    let profilesClient
    if ((import.meta as any).env.DEV) {
      try {
        await initializeHotReload();
      } catch (e) {
        console.warn("Could not initialize applet hot-reloading. This is only expected to work in a We context in dev mode.")
      }
    }
    let tokenResp;
    if (!isWeaveContext()) {
      console.log("adminPort is", adminPort);
      if (adminPort) {
        const url = `ws://localhost:${adminPort}`;
        console.log("connecting to admin port at:", url);
        const adminWebsocket = await AdminWebsocket.connect({
          url: new URL(url)
        });
        console.log("issuing token");
        tokenResp = await adminWebsocket.issueAppAuthenticationToken({
          installed_app_id: appId,
        });
        console.log("token", tokenResp);
        const x = await adminWebsocket.listApps({});
        console.log("apps", x);
        const cellIds = await adminWebsocket.listCellIds();
        console.log("CELL IDS", cellIds);
        await adminWebsocket.authorizeSigningCredentials(cellIds[0]);
      }
      console.log("appPort and Id is", appPort, appId);
      const params: AppWebsocketConnectionOptions = { url: new URL(url) };
      console.log("params", params);
      if (tokenResp) params.token = tokenResp.token;
      console.log("connecting to app port at:", params.url);
      client = await AppWebsocket.connect(params);
      console.log("client", client);
      profilesClient = new ProfilesClient(client, appId);
    }
    else {
      // const weClient = await WeaveClient.connect();
      weClient = await WeaveClient.connect(appletServices);

      // store set
      setWeaveClient(weClient)
      // weClient = await WeaveClient.connect();
      
      // switch (weClient?.renderInfo.type) {
      //   case "applet-view":
          switch (weClient?.renderInfo.view.type) {
            case "main":
              // here comes your rendering logic for the main view
              break;
            case "block":
              switch(weClient?.renderInfo.view.block) {
                case "active_boards":
                  currentView = "dashboard"
                  break;
                default:
                  throw new Error("Unknown applet-view block type:"+weClient?.renderInfo.view.block);
              }
              break;
            case "creatable":
              switch (weClient?.renderInfo.view.name) {
                case "Event":
                  currentView = "create-coordination-mini"
                case "Agreement":
                  currentView = "create-agreement-mini"
                  break;
              }
              break;
            case "asset":
              switch (weClient?.renderInfo.view.recordInfo.roleName) {
                case "whosin":
                  switch (weClient?.renderInfo.view.recordInfo.integrityZomeName) {
                    case "coordinator_integrity":
                      switch (weClient?.renderInfo.view.recordInfo.entryType) {
                        case "coordination":
                          // TODO: don't need to fetch all, just need to populate the store to keep track of 
                            // the correct client for the coordination
                          await refetchCoordinations(weClient?.renderInfo.appletClient)
                          currentView = "coordination"
                          currentHash = weClient?.renderInfo.view.wal.hrl[1]
                          // console.log("weClient?.renderInfo.view", weClient?.renderInfo.view)
                          // hrlWithContext = weClient?.renderInfo.view.hrlWithContext
                          break;
                        default:
                          throw new Error("Unknown entry type:"+weClient?.renderInfo.view.recordInfo.entryType);
                      }
                      break;
                    default:
                      throw new Error("Unknown integrity zome:"+weClient?.renderInfo.view.recordInfo.integrityZomeName);
                  }
                  break;
                default:
                  throw new Error("Unknown role name:"+weClient?.renderInfo.view.recordInfo.roleName);
              }
              break;
            default:
              throw new Error("Unsupported applet-view type");
          }
      //     break;
      //   case "cross-applet-view":
      //     currentView = "dashboard"
      //     switch (this.weClient?.renderInfo.view.type) {
      //       case "main":
      //         // here comes your rendering logic for the cross-applet main view
      //         //break;
      //       case "block":
      //         //
      //         //break;
      //       default:
      //         throw new Error("Unknown cross-applet-view render type.")
      //     }
      //     break;
      //   default:
      //     throw new Error("Unknown render view type");

      // }
      
      //@ts-ignore

      if (weClient?.renderInfo.type == "applet-view") {
        client = weClient?.renderInfo.appletClient;
        profilesClient = weClient?.renderInfo.profilesClient;
      } else {
        applets = Array.from(weClient?.renderInfo.applets.entries());
        const firstApplet = applets[0];
        console.log("we client 2", firstApplet)
        client = firstApplet[1].appletClient;
        profilesClient = firstApplet[1].profilesClient;
      }
      //@ts-ignore
    }
    profilesStore = new ProfilesStore(profilesClient);
    connected = true
  }

  onMount(async () => {
    await setDefaultPalette();
    await initialize()
    await checkForNotifier()
    if (typeof document !== 'undefined') {
      // document.documentElement.style.setProperty("--dynamic-primary", "#6200ee");
      // document.body.style.background = "black";
    }
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
    getApplets: () => applets,
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

<!-- <div id="colorPalette" style="margin-top: 100px; position: fixed; top: 2000; left: 0; z-index: 1000; display: flex; flex-direction: column; padding: 10px; height: 100px; width: 100px;">
  {#if $colorPalette}
    {#each Object.keys($colorPalette) as key}
      <div style='background-color: rgb({$colorPalette[key].rgb.join(',')})'>{key}: ({$colorPalette[key].rgb.join(',')})</div>
    {/each}
  {/if}
</div> -->

{#if client || applets != undefined}
{#if profilesStore || applets != undefined}
  <profiles-context store="{profilesStore}">
    <profile-prompt>
      {#if !isWeaveContext() || (isWeaveContext() && weClient?.renderInfo.view.type != "asset")}
      <NotificationsHandler></NotificationsHandler>
      <main style="width: 100vw;">
          {#if currentView != "create-coordination-mini" && currentView != "create-agreement-mini" && currentView != "create-project-mini"}
            <Header></Header>
          {/if}

          {#if !loading && !notifier && allNotifiers?.length > 0 && !(["notifier", "notificant", "home", "create-coordination"].includes(String(currentView)))}
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
          {:else if ["create-coordination-mini"].includes(currentView)}
            <div style="padding: 10px;">
              <CreateCoordination></CreateCoordination>
            </div>
          {:else if currentView == "create-agreement-mini"}
            <div style="padding: 10px;">
              <CreateCoordination agreementType="agreement"></CreateCoordination>
            </div>
          {:else if currentView == "create-project-mini"}
            <div style="padding: 10px;">
              <CreateCoordination agreementType="project"></CreateCoordination>
            </div>
          {:else if ["create-event", "create-agreement", "create-project"].includes(currentView)}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <div class="white-container" style="display: flex; flex-direction: column; margin-top: 30px;">
                <CreateCoordination agreementType={currentView.replace("create-", "")}></CreateCoordination>
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
          {:else if currentView == "calendar"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <Calendar></Calendar>
            </span>
          {:else if currentView == "notificant"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <CreateContact 
              on:my-notifier={()=>{
                // navigate("all-coordinations")
                // reload page
                location.reload()
              }}
              ></CreateContact>
            </span>
          {:else}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <!-- <Instructions></Instructions> -->
              <Calendar></Calendar>
            </span>
          {/if}

        <!-- <footer style="margin: 10px;">
          <SvgIcon icon=faBug size="24" color="#000000" />
          <a href="https://docs.google.com/forms/d/e/1FAIpQLSdzwS5D1HP3Eq6JV2lSD2cTXZoVTJJR2b7vEuAKgk9izVFRIw/viewform" target="_blank" class="feedback-button">
            <span>Submit feedback</span>
          </a>
          :)
        {#if !isWeaveContext() && dna && !loading && currentView != "instructions" && currentView != ""}
        <br>
        <small>
          <img class="holochain-logo" src={Holochain} alt="holochain logo"/>
          Private Holochain network: {dna}
        </small>
        {/if}
        </footer> -->
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
  <Sync {client}/>
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
