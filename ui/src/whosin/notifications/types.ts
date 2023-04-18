import type { 
  Record, 
  ActionHash,
  DnaHash,
  SignedActionHashed,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type NotificationsSignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

export type EntryTypes =
 | ({ type: 'SentNotification'; } & SentNotification)
 | ({ type: 'Contact'; } & Contact)
 | ({ type: 'TwilioCredentials'; } & TwilioCredentials)
 | ({  type: 'Contacts'; } & Contacts);



export interface Contacts { 
  agent_pub_key: AgentPubKey;

  text_number: string | undefined;

  whatsapp_number: string | undefined;

  email_address: string | undefined;
}




export interface TwilioCredentials { 
  account_sid: string;

  auth_token: string;

  from_number: string;
}




export interface Contact { 
  agent_pub_key: AgentPubKey;

  text_number: string | undefined;

  whatsapp_number: string | undefined;

  email_address: string | undefined;
}




export interface SentNotification { 
  unique_data: string;
}

