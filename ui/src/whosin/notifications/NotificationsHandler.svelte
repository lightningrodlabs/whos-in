<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import { decode } from '@msgpack/msgpack';
  import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash, AnyDhtHash } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { TwilioCredentials, SentNotification } from './types';
  import '@material/mwc-circular-progress';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-snackbar';
  import '@material/mwc-icon-button';
  
  // export let twilioCredentialsHash: ActionHash;
  
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  let loading = true;
  let error: any = undefined;
  
  let record: Record | undefined;
  let twilioCredentials;//: TwilioCredentials | undefined;
  let all_participants = [];

  let editing = false;
  
  let errorSnackbar: Snackbar;
    
  $: editing,  error, loading, record, twilioCredentials;
  
  onMount(async () => {
    // receive a dispatch signal from the backend
    client.on(
      'signal', 
      (signal) => {
        // wait a few seconds before executing
        setTimeout(() => {
          try {
            if (signal.zome_name === 'notifications') {
              fetchCoordroles(signal.payload).then(activated => {
                if (activated) {
                  fetchCoordination(signal.payload).then(coordination => {
                    was_it_sent(signal.payload + " activated").then(sent => {
                      console.log(sent)
                      if (!sent) {
                        fetchTwilioCredentials().then(() => {
                          all_participants.forEach(participant => {
                            fetchContact(participant).then(contacts => {
                              var contact = contacts.filter(c => {
                                return JSON.stringify(c.agent_pub_key) == JSON.stringify(participant)
                              })
                              if (contact) {
                                sendText(contact[contact.length-1]["text_number"], coordination["title"] + " is activated!", twilioCredentials["account_sid"], twilioCredentials["auth_token"], twilioCredentials["from_number"]);
                                createSentNotification(signal.payload + " activated")
                              }
                            })
                          })
                        })
                      }
                    })
                  })
                }
              })
            }
          } catch (e) {
            console.log(e.data.data)
          }
        }, 1000);
      });
    });

  async function fetchTwilioCredentials() {
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'notifications',
        fn_name: 'get_twilio_credentials',
        payload: null,
      });
      if (record) {
        twilioCredentials = decode((record.entry as any).Present.entry) as TwilioCredentials;
      }
    } catch (e) {
      console.log(e)
    }
  }

  async function createSentNotification(uniqueData) {  
    const sentNotificationEntry: SentNotification = { 
      unique_data: uniqueData!,
    };
    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'notifications',
        fn_name: 'create_sent_notification',
        payload: sentNotificationEntry,
      });
    } catch (e) {
      console.log(e.data.data)
    }
  }

  async function was_it_sent(unique_data: String) {    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'notifications',
        fn_name: 'retrieve_sent_notifications',
        payload: null,
      });
      if (record) {
        // console.log(record)
        let output = false
        for (var i = 0; i < record.length; i++) {
          let sentNotification = decode((record[i].entry as any).Present.entry) as SentNotification;
          if (sentNotification["unique_data"] == unique_data) {
            output = true
          }
        }
        return output
      }
    } catch (e) {
      error = e;
    }
  }

  async function fetchCoordroles(actionHash) {
    let record;
    let activated;
    try {
      record = await client
      .callZome({
          cap_secret: null,
          role_name: 'whosin',
          zome_name: 'coordinator',
          fn_name: 'get_coordroles_for_coordination',
          payload: actionHash,
      });
      if (record) {
        activated = true;
        record.forEach(role => {
          if (role.participants < decode(role.coordrole.entry.Present.entry)["minimum"]) {
            activated = false;
          }

          role.participants_details.forEach(participant => {
            all_participants.push(participant);
          })
        })
        return activated;
      }
    }
    catch (e) {
      error = e;
    }
  };

  async function fetchCoordination(coordinationHash) {
    record = undefined;      
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'coordinator',
        fn_name: 'get_coordination',
        payload: coordinationHash,
      });
      if (record) {
        return decode((record.entry as any).Present.entry);
      }
    } catch (e) {
      error = e;
    }
  }

  async function fetchContact(agent_pub_key) { 
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'whosin',
        zome_name: 'notifications',
        fn_name: 'get_contacts',
        payload: agent_pub_key,
      });
      console.log(record)
      if (record) {
        // return record
        let all_contacts = [];
        record.forEach(element => {
          all_contacts.push(decode((element.entry as any).Present.entry));          
        });
        return all_contacts
      }
    } catch (e) {
      error = e.data.data;
    }
  }

  async function sendText(to, message, account_sid, auth_token, from) {
    fetch('https://api.twilio.com/2010-04-01/Accounts/' + account_sid + '/Messages.json', {
      method: 'POST',
      headers: {
        'Authorization': 'Basic ' + btoa(account_sid + ':' + auth_token),
      },
      body: new URLSearchParams({
        'To': to,
        'From': from,
        'Body': message
      })
    });
  }
  
  // async function deleteTwilioCredentials() {
  //   try {
  //     await client.callZome({
  //       cap_secret: null,
  //       role_name: 'whosin',
  //       zome_name: 'notifications',
  //       fn_name: 'delete_twilio_credentials',
  //       payload: twilioCredentialsHash,
  //     });
  //     dispatch('twilio-credentials-deleted', { twilioCredentialsHash: twilioCredentialsHash });
  //   } catch (e: any) {
  //     errorSnackbar.labelText = `Error deleting the twilio credentials: ${e.data.data}`;
  //     errorSnackbar.show();
  //   }
  // }
  </script>