import { asyncDerived, pipe, sliceAndJoin, toPromise } from '@holochain-open-dev/stores';
import { LazyHoloHashMap } from '@holochain-open-dev/utils';
import type { AppletHash, AppletServices, WAL,AssetInfo, WeServices } from '@lightningrodlabs/we-applet';
import type { AppAgentClient, RoleName, ZomeName } from '@holochain/client';
import { getMyDna, hrlWithContextToB64 } from './util';
import { AppWebsocket, AppAgentWebsocket, AdminWebsocket } from '@holochain/client';
import type { Coordination } from './whosin/coordinator/types';
import { decode } from '@msgpack/msgpack';

const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
const adminPort = import.meta.env.VITE_ADMIN_PORT
const url = `ws://localhost:${appPort}`;

const ROLE_NAME = "whosin"
const ZOME_NAME = "coordinator"
const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'converge'

const ICON = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" style="enable-background:new 0 0 64 64" xml:space="preserve"><path d="M6 12c0-3.3 2.7-6 6-6h40c3.3 0 6 2.7 6 6v40c0 3.3-2.7 6-6 6H12c-3.3 0-6-2.7-6-6V12z" style="fill:%23fff"/><path d="M4 12c0-4.4 3.6-8 8-8h8v16H8v12h12v12H8v2c0 2.8 5.1 5.1 12 5.8V44h12v7.4c4.4-.7 8.5-2 12-3.8V44h5.6c4-3.3 6.4-7.5 6.4-12h2v20c0 3.3-2.7 6-6 6h-8v2H12c-4.4 0-8-3.6-8-8V12zm28 20v12h12V32H32zm12 0h12V20H44v12zm0-12V8H32v12h12zm-12 0H20v12h12V20z" style="fill:%23acbdc5"/><path d="M32 56H20v-4.2c1.3.1 2.6.2 4 .2 2.8 0 5.4-.2 8-.6V56zm12-8.4V56h12V44h-6.4c-1.6 1.3-3.5 2.6-5.6 3.6z" style="fill:%23597380"/><path d="M20 4h32c4.4 0 8 3.6 8 8v40c0 4.4-3.6 8-8 8h-8v-4h8c2.2 0 4-1.8 4-4V12c0-2.2-1.8-4-4-4H20V4z" style="fill-rule:evenodd;clip-rule:evenodd;fill:%23314a52"/></svg>'
const CARD_ICON_SRC = `data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M0 96C0 60.7 28.7 32 64 32H448c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96zm64 0v64h64V96H64zm384 0H192v64H448V96zM64 224v64h64V224H64zm384 0H192v64H448V224zM64 352v64h64V352H64zm384 0H192v64H448V352z"/></svg>`
const MINILOGO = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 50 50"><circle cx="15" cy="15" r="10" style="fill:red;" /><path d="M 15 25 Q 15 20, 15 30" style="stroke:black; fill:transparent;" /><circle cx="35" cy="15" r="10" style="fill:blue;" /><path d="M 35 25 Q 35 20, 35 30" style="stroke:black; fill:transparent;" /></svg>'

export const appletServices: AppletServices = {
    // Types of attachment that this Applet offers for other Applets to attach
    creatables: {
      'Coordination': {
        label: "Deliberation",
        icon_src: MINILOGO,
      }
    },
    // Types of UI widgets/blocks that this Applet supports
    blockTypes: {
      'my_deliberations': {
        label: 'My Deliberations',
        icon_src: 
        `<svg xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 512 512"><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M0 96C0 60.7 28.7 32 64 32H448c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96zm64 0v64h64V96H64zm384 0H192v64H448V96zM64 224v64h64V224H64zm384 0H192v64H448V224zM64 352v64h64V352H64zm384 0H192v64H448V352z"/></svg>`,
        view: "applet-view",
      },      
    },
    bindAsset: async (appletClient: AppAgentClient,
      srcWal: WAL, dstWal: WAL): Promise<void> => {
      console.log("Bind requested.  Src:", srcWal, "  Dst:", dstWal)
    },  
    getAssetInfo: async (
      appletClient: AppAgentClient,
      roleName: RoleName,
      integrityZomeName: ZomeName,
      entryType: string,
      wal: WAL
    ): Promise<AssetInfo | undefined> => {
        let dnaHash = await getMyDna(ROLE_NAME, appletClient)
        let coordination: Coordination;
        let record: any;

        try {
          record = await appletClient.callZome({
            cap_secret: null,
            role_name: 'whosin',
            zome_name: 'coordinator',
            fn_name: 'get_coordination',
            payload: wal.hrl[1],
          });
          if (record) {
            // console.log("record", record)
            coordination = decode((record.entry as any).Present.entry) as Coordination;
          }
        } catch (e) {
          console.log(e)
        }

        return {
          icon_src: `data:image/svg+xml;utf8,${MINILOGO}`,
          name: coordination.title,
        };
    },
    search: async (
      appletClient: AppAgentClient,
      appletHash: AppletHash,
      weServices: WeServices,
      searchFilter: string
    ): Promise<Array<WAL>> => {
      let hashes: WAL[];
      let dnaHash = await getMyDna(ROLE_NAME, appletClient)

      try {
        const records = await appletClient.callZome({
          cap_secret: null,
          role_name: 'whosin',
          zome_name: 'coordinator',
          fn_name: 'search_all_coordinations',
          payload: searchFilter,
        });
        console.log("hashes", hashes)
        hashes = records
        .map((r) => ({ hrl: [dnaHash, r], context: {} }));
      } catch (e) {
        console.log(e)
      }
      return hashes
    },
};