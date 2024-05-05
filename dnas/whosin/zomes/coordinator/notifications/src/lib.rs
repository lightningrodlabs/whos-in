extern crate hc_zome_notifications_coordinator;
use hdk::prelude::{*};
#[derive(Serialize, Deserialize, Debug)]
pub struct CoordrolesOutput {
    coordrole: Record,
    participants: usize,
    participants_details: Vec<AgentPubKey>,
    committed: bool,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Coordination {
    pub title: String,
    pub description: String,
    pub starts_date: Option<Timestamp>,
    pub ends_date: Option<Timestamp>,
    pub signup_deadline: Option<Timestamp>,
    pub reminder_date: Option<Timestamp>,
    pub coordroles: Vec<ActionHash>,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Coordrole {
  pub title: String,
  pub description: String,
  pub minimum: i32,
  pub maximum: i32,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Contact {
    pub agent_pub_key: AgentPubKey,
    pub text_number: Option<String>,
    pub whatsapp_number: Option<String>,
    pub email_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationTip {
  pub retry_count: i32, //The number of times this notification has been retried. Use custom_handle_notification_tip to increment and limit.
  pub status: String, //"stop", "go" or "retry"
  pub message: String, //The message to send to the notificant
  pub notificants: Vec<AgentPubKey>, //The list of notificants to send the message to
  pub contacts: Vec<Contact>, //The list of contacts to send the message to. If left blank, contact details will automatically be retrieved based on the notificants.
  pub extra_context: String, //Any extra data that needs to be passed to the custom_handle_notification_tip function. For example, a hash of an entry.
  pub message_id: String, //A unique identifier of the message to send. For instance, a string containing a timestamp and message content. This is used to automtatically prevent duplicate messages. If left blank, there will be no prevention of duplicate notifications.
  pub destination: String, //Used for debugging. The name of the function that the data is being sent to.
}

// #[hdk_extern]
// pub fn custom_handle_notification_tip(data: String) -> ExternResult<String> {
//   // Ok(String::from("custom notification data"))
//   Ok(data)
// }

#[hdk_extern]
pub fn custom_handle_notification_tip(data: NotificationTip) -> ExternResult<NotificationTip> {
  let coordination_hash: ActionHash = ActionHash::try_from(data.extra_context.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))?;

  let mut output = NotificationTip {
    retry_count: data.retry_count.clone(),
    status: String::from("stop"),
    message: String::from(""),
    notificants: vec![],
    contacts: vec![],
    extra_context: coordination_hash.to_string(),
    message_id: String::from(""),
    destination: String::from("handle_notification_tip"),
  };

  let mut participants: Vec<AgentPubKey> = vec![];
  let mut activated: bool = false;
  let coordination: Coordination;

  // START GET COMMITTERS FOR COORDINATION AND FIND ACTIVATION
  let zome_call_response = call_remote(
    agent_info().unwrap().agent_latest_pubkey.into(),
    "coordinator",
    FunctionName(String::from("get_coordroles_for_coordination")),
    None,
    coordination_hash.clone(),
  )?;
  // emit_signal(zome_call_response.clone())?;
  match zome_call_response {
      ZomeCallResponse::Ok(result) => { // ExternIO is a wrapper around a byte array
        let record: Vec<CoordrolesOutput> = result.decode().map_err(|err| {
          // emit_signal(err.clone());
          wasm_error!(String::from(err))
        })?; // Deserialize byte array

        // let mut coordroles: Vec<Coordrole> = vec![];

        for item in record.iter() {
          emit_signal(item.clone())?;
          let coordrole: Coordrole = item
            .coordrole
            .entry()
            .to_app_option()
            .map_err(|err| wasm_error!(err))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "Could not deserialize element to Coordrole.".into(),
            )))?;
          // coordroles.push(coordrole);

          // emit_signal(coordrole.clone())?;

          if item.participants as i32 >= coordrole.minimum {
            activated = true;
          }
        }

        if activated {
          for item in record.iter() {
            for participant in item.participants_details.iter() {
              participants.push(participant.clone());
              emit_signal(participant)?;
            }
          }
        }

        // emit_signal(participants.clone())?;
      },
      _ => {
        // Ok(false)
      },
  }
  // END GET COMMITTERS FOR COORDINATION AND FIND ACTIVATION

  // START CHECK SENT NOTIFICATIONS
  let zome_call_response = call_remote(
    agent_info().unwrap().agent_latest_pubkey.into(),
    "notifications",
    FunctionName(String::from("retrieve_sent_notifications")),
    None,
    {},
  )?;
  emit_signal(zome_call_response.clone())?;
  match zome_call_response {
      ZomeCallResponse::Ok(result) => { // ExternIO is a wrapper around a byte array
        let record: Vec<Record> = result.decode().map_err(|err| {
          emit_signal(err.clone());
          wasm_error!(String::from(err))
        })?; // Deserialize byte array
        // emit_signal(record)?;
      },
      _ => {
        emit_signal("no")?;
        // emit_signal(zome_call_response)?;
        // Ok(false)
      },
  }
  // END CHECK SENT NOTIFICATIONS

  // START GET COORDINATION DETAILS
  let zome_call_response = call_remote(
    agent_info().unwrap().agent_latest_pubkey.into(),
    "coordinator",
    FunctionName(String::from("get_coordination")),
    None,
    coordination_hash.clone(),
  )?;
  match zome_call_response {
      ZomeCallResponse::Ok(result) => { // ExternIO is a wrapper around a byte array
        let record: Record = result.decode().map_err(|err| wasm_error!(String::from(err)))?; // Deserialize byte array
        let coordination_record: Coordination = record
          .entry()
          .to_app_option()
          .map_err(|err| wasm_error!(err))?
          .ok_or(wasm_error!(WasmErrorInner::Guest(
              "Could not deserialize element to Coordination.".into(),
          )))?;
        coordination = coordination_record;
        // emit_signal(coordination.clone())?;

        let status;
        let message: String;
        let mut retry_count = data.retry_count;
        
        if activated {
          status = String::from("send");
          message = format!("Action activated: \"{}\"", coordination.title);
        } else {
          retry_count += 1;
          status = String::from("retry");
          message = String::from("");
        }

        let message_id = format!("{} {}", data.message, data.extra_context);

        output = NotificationTip {
          retry_count: retry_count,
          status: status,
          message: message,
          notificants: participants,
          contacts: vec![],
          extra_context: coordination_hash.to_string(),
          message_id: message_id,
          destination: String::from("handle_notification_tip"),
        };
      
        emit_signal(output.clone())?;
      
      },
      _ => {
        emit_signal(false)?;
      },
  }
  // END GET COORDINATION DETAILS

  Ok(output)

}