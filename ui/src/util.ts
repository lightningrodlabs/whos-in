import { decodeHashFromBase64, encodeHashToBase64, type AppClient, type EntryHash, type DnaHash, CellType } from "@holochain/client";
import type { HrlB64WithContext, HrlWithContext } from "@lightningrodlabs/we-applet";

export function onVisible(element, callback) {
    new IntersectionObserver((entries, observer) => {
        entries.forEach(entry => {
        if(entry.intersectionRatio > 0) {
            callback(element);
        }
        });
    }).observe(element);
}

export type AppletHash = EntryHash;
// export function appletHashFromAppId(appId: string): AppletHash {
//   if (!appId.startsWith('applet#')) {
//     throw new Error('Invalid appId format');
//   }
//   const base64Hash = appId.slice(7); // Remove 'applet#' prefix
//   return decodeHashFromBase64(base64Hash);
// }

export function appletHashFromAppId(installedAppId: string): AppletHash {
  return decodeHashFromBase64(installedAppId.slice(7).replace(/[a-z]\$/g, (match) => match[0].toUpperCase()));
}

export function hrlWithContextToB64(hrl: HrlWithContext): HrlB64WithContext {
  return {
    hrl: [encodeHashToBase64(hrl.hrl[0]), encodeHashToBase64(hrl.hrl[1])],
    context: hrl.context === undefined ? 'null' : JSON.stringify(hrl.context),
  };
}
  
export function hrlB64WithContextToRaw(hrlB64: HrlB64WithContext): HrlWithContext {
  let context: any
  try {
    context = JSON.parse(hrlB64.context)
  } catch (e) {
    console.log("error", e)
  }

  return {
    hrl: [decodeHashFromBase64(hrlB64.hrl[0]), decodeHashFromBase64(hrlB64.hrl[1])],
    context,
  };
}

export type WALUrl = string

export const hashEqual = (a:EntryHash, b:EntryHash) : boolean => {
  if (!a || !b) {
    return !a && !b
  }
  for (let i = a.length; -1 < i; i -= 1) {
    if ((a[i] !== b[i])) return false;
  }
  return true;
}

export const getMyDna = async (role:string, client: AppClient) : Promise<DnaHash>  => {
  const appInfo = await client.appInfo();
  const dnaHash = (appInfo.cell_info[role][0] as any)[
    CellType.Provisioned
  ].cell_id[0];
  return dnaHash
} 

export const getCoordinationLabel = (coordination: any) : any => {
    const currentTime = new Date().getTime() * 1000;

    if (coordination.ends_date && coordination.ends_date < currentTime) {
        return {
            title: 'Expired',
            color: 'rgba(255, 255, 255, 0.5)'//'#ff0000'
        };
    } else if (coordination.totalUnderMin >= coordination.totalMin && coordination.starts_date && coordination.starts_date < currentTime) {
        return {
            title: 'Happening today',
            color: 'rgba(47, 97, 224, 0.75)'//'#cd1dff'
        };
    } else if (coordination.totalMin > 0 && coordination.totalUnderMin < coordination.totalMin && coordination.signup_deadline && coordination.signup_deadline < currentTime) {
        return {
            title: 'Did not reach minimum participation',
            color: 'gray'
        };
    } else if (coordination.totalMin > 0 && coordination.totalUnderMin < coordination.totalMin) {
        return {
            title: 'Gathering participation',
            color: 'rgba(255, 243, 17, 0.5)'
        };
    } else if (coordination.totalUnderMin >= coordination.totalMin) {
        return {
            title: 'Active',
            color: 'rgba(37, 255, 17, 0.5)'//'#57ca01'
        };
    }

    return null;
}
