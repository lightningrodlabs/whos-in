extern crate hc_zome_notifications;
use hdk::prelude::*;
// use notifications_integrity::*;
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
    pub happening_date: Option<Timestamp>,
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

#[hdk_extern]
pub fn custom_handle_notification_tip(data: String) -> ExternResult<String> {
  // Ok(String::from("custom notification data"))
  Ok(data)
}

// #[hdk_extern]
// pub fn validate_notification_tip(data: AnyLinkableHash) -> ExternResult<bool> {
//   // emit_signal("hello")?;
//   // let delay_duration = Duration::from_secs(1); // Delay duration of 1 second
//   // std::thread::sleep(delay_duration);
//   // emit_signal("world!")?;

//   let mut participants: Vec<AgentPubKey> = vec![];
//   let mut activated: bool = false;

//   let zome_call_response = call_remote(
//     agent_info().unwrap().agent_latest_pubkey.into(),
//     "coordinator",
//     FunctionName(String::from("get_coordination")),
//     None,
//     data.clone(),
//   )?;
//   match zome_call_response {
//       ZomeCallResponse::Ok(result) => { // ExternIO is a wrapper around a byte array
//         let record: Record = result.decode().map_err(|err| wasm_error!(String::from(err)))?; // Deserialize byte array
//         let coordination: Coordination = record
//           .entry()
//           .to_app_option()
//           .map_err(|err| wasm_error!(err))?
//           .ok_or(wasm_error!(WasmErrorInner::Guest(
//               "Could not deserialize element to Coordination.".into(),
//           )))?;
//         emit_signal(coordination)?;
//       },
//       _ => {
//         // Ok(false)
//       },
//   }

//   let zome_call_response = call_remote(
//     agent_info().unwrap().agent_latest_pubkey.into(),
//     "coordinator",
//     FunctionName(String::from("get_coordroles_for_coordination")),
//     None,
//     data.clone(),
//   )?;
//   emit_signal(zome_call_response.clone())?;
//   match zome_call_response {
//       ZomeCallResponse::Ok(result) => { // ExternIO is a wrapper around a byte array
//         let record: Vec<CoordrolesOutput> = result.decode().map_err(|err| {
//           emit_signal(err.clone());
//           wasm_error!(String::from(err))
//         })?; // Deserialize byte array

//         // let mut coordroles: Vec<Coordrole> = vec![];

//         for item in record.iter() {
//           emit_signal(item.clone())?;
//           let coordrole: Coordrole = item
//             .coordrole
//             .entry()
//             .to_app_option()
//             .map_err(|err| wasm_error!(err))?
//             .ok_or(wasm_error!(WasmErrorInner::Guest(
//                 "Could not deserialize element to Coordrole.".into(),
//             )))?;
//           // coordroles.push(coordrole);

//           emit_signal(coordrole.clone())?;

//           if item.participants as i32 >= coordrole.minimum-1 {
//             activated = true;
//           }
//         }

//         // if activated {
//         //   for item in record.iter() {
//         //     for participant in item.participants_details.iter() {
//         //       // participants.push(participant.clone());
//         //       emit_signal(participant)?;
//         //     }
//         //   }
//         // }

//         // emit_signal(participants)?;
//       },
//       _ => {
//         // Ok(false)
//       },
//   }


//   let zome_call_response = call_remote(
//     agent_info().unwrap().agent_latest_pubkey.into(),
//     "notifications",
//     FunctionName(String::from("retrieve_sent_notifications")),
//     None,
//     {},
//   )?;
//   emit_signal(zome_call_response.clone())?;
//   match zome_call_response {
//       ZomeCallResponse::Ok(result) => { // ExternIO is a wrapper around a byte array
//         let record: Vec<Record> = result.decode().map_err(|err| {
//           emit_signal(err.clone());
//           wasm_error!(String::from(err))
//         })?; // Deserialize byte array
//         emit_signal(record)?;
//       },
//       _ => {
//         emit_signal("no")?;
//         emit_signal(zome_call_response)?;
//         // Ok(false)
//       },
//   }


 
//   Ok(activated)
// }