import type { 
  Record, 
  ActionHash,
  DnaHash,
  SignedActionHashed,
  EntryHash, 
  AgentPubKey
} from '@holochain/client';

import type { HrlB64WithContext, Hrl } from '@theweave/api';

export type CoordinatorSignal = {
  type: 'EntryCreated';
  action: SignedActionHashed;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed;
  link_type: string;
};

export type EntryTypes =
 | ({ type: 'TwilioCredentials'; } & TwilioCredentials)
 | ({ type: 'Contact'; } & Contact)
 | ({ type: 'Coordrole'; } & Coordrole)
 | ({  type: 'Coordination'; } & Coordination);



export interface Coordination { 
  title: string;

  description: string;

  coordination_type: string | null;

  starts_date: number | null;

  ends_date: number | null;

  signup_deadline: number | null;

  reminder_date: number | null;

  coordroles: Array<Coordrole>;

  attachments?: HrlB64WithContext[];


  // added by the client
  totalParticipants: number | null;

  totalMin: number | null;

  totalUnderMin: number | null;
}



export interface Coordrole { 
  title: string;

  description: string;

  minimum: number;

  maximum: number;
}




export interface Contact { 
  agent_pub_key: AgentPubKey;

  text_number: string | null;

  whatsapp_number: string | null;

  email: string | null;
}




export interface TwilioCredentials { 
  account_sid: string;

  auth_token: string;
}


export type AvailabilitySlot = {
  status: number,
  time: number,
  // duration: number,
}