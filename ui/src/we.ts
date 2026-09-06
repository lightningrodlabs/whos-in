import { asyncDerived, pipe, sliceAndJoin, toPromise } from '@holochain-open-dev/stores';
// 0.21: the HoloHashMap family moved from @holochain-open-dev/utils into the client.
import { LazyHoloHashMap } from '@holochain/client';
import type { AppletHash, AppletServices, AssetInfo, WAL, RecordInfo, WeaveServices } from '@theweave/api';
import type { RoleName, ZomeName, AppClient } from '@holochain/client';
import { getMyDna, hrlWithContextToB64 } from './util';
import type { Coordination } from './whosin/coordinator/types';
import { decode } from '@msgpack/msgpack';

const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
const adminPort = import.meta.env.VITE_ADMIN_PORT
const url = `ws://localhost:${appPort}`;

const ROLE_NAME = "whosin"
const ZOME_NAME = "coordinator"
const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'whosin'

const ICON = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" style="enable-background:new 0 0 64 64" xml:space="preserve"><path d="M6 12c0-3.3 2.7-6 6-6h40c3.3 0 6 2.7 6 6v40c0 3.3-2.7 6-6 6H12c-3.3 0-6-2.7-6-6V12z" style="fill:%23fff"/><path d="M4 12c0-4.4 3.6-8 8-8h8v16H8v12h12v12H8v2c0 2.8 5.1 5.1 12 5.8V44h12v7.4c4.4-.7 8.5-2 12-3.8V44h5.6c4-3.3 6.4-7.5 6.4-12h2v20c0 3.3-2.7 6-6 6h-8v2H12c-4.4 0-8-3.6-8-8V12zm28 20v12h12V32H32zm12 0h12V20H44v12zm0-12V8H32v12h12zm-12 0H20v12h12V20z" style="fill:%23acbdc5"/><path d="M32 56H20v-4.2c1.3.1 2.6.2 4 .2 2.8 0 5.4-.2 8-.6V56zm12-8.4V56h12V44h-6.4c-1.6 1.3-3.5 2.6-5.6 3.6z" style="fill:%23597380"/><path d="M20 4h32c4.4 0 8 3.6 8 8v40c0 4.4-3.6 8-8 8h-8v-4h8c2.2 0 4-1.8 4-4V12c0-2.2-1.8-4-4-4H20V4z" style="fill-rule:evenodd;clip-rule:evenodd;fill:%23314a52"/></svg>'
const CARD_ICON_SRC = `data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M0 96C0 60.7 28.7 32 64 32H448c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96zm64 0v64h64V96H64zm384 0H192v64H448V96zM64 224v64h64V224H64zm384 0H192v64H448V224zM64 352v64h64V352H64zm384 0H192v64H448V352z"/></svg>`
const MINILOGO = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 50 50"><circle cx="15" cy="15" r="10" style="fill:red;" /><path d="M 15 25 Q 15 20, 15 30" style="stroke:black; fill:transparent;" /><circle cx="35" cy="15" r="10" style="fill:blue;" /><path d="M 35 25 Q 35 20, 35 30" style="stroke:black; fill:transparent;" /></svg>'
const EVENTLOGO = `<svg xmlns="http://www.w3.org/2000/svg" fill="%23357cff" viewBox="0 0 448 512"><path d="M128 0c17.7 0 32 14.3 32 32V64H288V32c0-17.7 14.3-32 32-32s32 14.3 32 32V64h48c26.5 0 48 21.5 48 48v48H0V112C0 85.5 21.5 64 48 64H96V32c0-17.7 14.3-32 32-32zM0 192H448V464c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V192zm64 80v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V272c0-8.8-7.2-16-16-16H80c-8.8 0-16 7.2-16 16zm128 0v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V272c0-8.8-7.2-16-16-16H208c-8.8 0-16 7.2-16 16zm144-16c-8.8 0-16 7.2-16 16v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V272c0-8.8-7.2-16-16-16H336zM64 400v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V400c0-8.8-7.2-16-16-16H80c-8.8 0-16 7.2-16 16zm144-16c-8.8 0-16 7.2-16 16v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V400c0-8.8-7.2-16-16-16H208zm112 16v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V400c0-8.8-7.2-16-16-16H336c-8.8 0-16 7.2-16 16z"/></svg>`
const PROJECTLOGO = `<svg xmlns="http://www.w3.org/2000/svg" fill="%23ff951d" viewBox="0 0 576 512"><path d="M413.5 237.5c-28.2 4.8-58.2-3.6-80-25.4l-38.1-38.1C280.4 159 272 138.8 272 117.6l0-12.1L192.3 62c-5.3-2.9-8.6-8.6-8.3-14.7s3.9-11.5 9.5-14l47.2-21C259.1 4.2 279 0 299.2 0l18.1 0c36.7 0 72 14 98.7 39.1l44.6 42c24.2 22.8 33.2 55.7 26.6 86L503 183l8-8c9.4-9.4 24.6-9.4 33.9 0l24 24c9.4 9.4 9.4 24.6 0 33.9l-88 88c-9.4 9.4-24.6 9.4-33.9 0l-24-24c-9.4-9.4-9.4-24.6 0-33.9l8-8-17.5-17.5zM27.4 377.1L260.9 182.6c3.5 4.9 7.5 9.6 11.8 14l38.1 38.1c6 6 12.4 11.2 19.2 15.7L134.9 484.6c-14.5 17.4-36 27.4-58.6 27.4C34.1 512 0 477.8 0 435.7c0-22.6 10.1-44.1 27.4-58.6z"/></svg>`
const AGREEMENTLOGO = `<svg xmlns="http://www.w3.org/2000/svg" fill="%235301ae" viewBox="0 0 640 512"><path d="M323.4 85.2l-96.8 78.4c-16.1 13-19.2 36.4-7 53.1c12.9 17.8 38 21.3 55.3 7.8l99.3-77.2c7-5.4 17-4.2 22.5 2.8s4.2 17-2.8 22.5l-20.9 16.2L512 316.8 512 128l-.7 0-3.9-2.5L434.8 79c-15.3-9.8-33.2-15-51.4-15c-21.8 0-43 7.5-60 21.2zm22.8 124.4l-51.7 40.2C263 274.4 217.3 268 193.7 235.6c-22.2-30.5-16.6-73.1 12.7-96.8l83.2-67.3c-11.6-4.9-24.1-7.4-36.8-7.4C234 64 215.7 69.6 200 80l-72 48 0 224 28.2 0 91.4 83.4c19.6 17.9 49.9 16.5 67.8-3.1c5.5-6.1 9.2-13.2 11.1-20.6l17 15.6c19.5 17.9 49.9 16.6 67.8-2.9c4.5-4.9 7.8-10.6 9.9-16.5c19.4 13 45.8 10.3 62.1-7.5c17.9-19.5 16.6-49.9-2.9-67.8l-134.2-123zM16 128c-8.8 0-16 7.2-16 16L0 352c0 17.7 14.3 32 32 32l32 0c17.7 0 32-14.3 32-32l0-224-80 0zM48 320a16 16 0 1 1 0 32 16 16 0 1 1 0-32zM544 128l0 224c0 17.7 14.3 32 32 32l32 0c17.7 0 32-14.3 32-32l0-208c0-8.8-7.2-16-16-16l-80 0zm32 208a16 16 0 1 1 32 0 16 16 0 1 1 -32 0z"/></svg>`

const MINILOGO2 = '<svg width="200" height="100" xmlns="http://www.w3.org/2000/svg"><rect x="10" y="10" width="180" height="80" fill="darkgreen" stroke="lightgreen" stroke-width="5"/></svg>'

export const appletServices: AppletServices = {
    // Types of attachment that this Applet offers for other Applets to attach
    creatables: {
      'Event': {
        label: "Event",
        icon_src: 'data:image/svg+xml;utf8,' + EVENTLOGO,
        width: 'large',
        height: 'large',
      },
      'Agreement': { 
        label: "Agreement",
        icon_src: 'data:image/svg+xml;utf8,' + AGREEMENTLOGO,
        width: 'large',
        height: 'large',
      }
    },
    // bindAsset: async (appletClient: AppClient,
    //   srcWal: WAL, dstWal: WAL): Promise<void> => {
    //   console.log("Bind requested.  Src:", srcWal, "  Dst:", dstWal)
    // },  
    getAssetInfo: async (
      appletClient: AppClient,
      wal: WAL,
      recordInfo: RecordInfo
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
            coordination = decode((record.entry as any).Present.entry) as Coordination;
          }
        } catch (e) {
          console.log(e)
        }

        let logo = EVENTLOGO
        if (coordination.coordination_type == "Agreement") {
          logo = AGREEMENTLOGO
        } else if (coordination.coordination_type == "Project") {
          logo = PROJECTLOGO
        }

        return {
          icon_src: `data:image/svg+xml;utf8,${logo}`,
          name: coordination.title + " (" + coordination.coordination_type + ")",
        };
    },
    search: async (
      appletClient: AppClient,
      appletHash: AppletHash,
      weServices: WeaveServices,
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