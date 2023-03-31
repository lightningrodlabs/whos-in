use hdk::prelude::*;
use coordinator_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddNotifierForNotificationRecipientInput {
    pub base_notification_recipient: AgentPubKey,
    pub target_notifier: AgentPubKey,
}
#[hdk_extern]
pub fn add_notifier_for_notification_recipient(input: AddNotifierForNotificationRecipientInput) -> ExternResult<()> {
    create_link(input.base_notification_recipient.clone(), input.target_notifier.clone(), LinkTypes::NotificationRecipientToNotifiers, ())?;
    create_link(input.target_notifier, input.base_notification_recipient, LinkTypes::NotifierToNotificationRecipients, ())?;

    Ok(())    
}

#[hdk_extern]
pub fn get_notifiers_for_notification_recipient(notification_recipient: AgentPubKey) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(notification_recipient, LinkTypes::NotificationRecipientToNotifiers, None)?;
    
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
        .collect();

    Ok(agents)
}


#[hdk_extern]
pub fn get_notification_recipients_for_notifier(notifier: AgentPubKey) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(notifier, LinkTypes::NotifierToNotificationRecipients, None)?;
    
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
        .collect();

    Ok(agents)
}
        
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveNotifierForNotificationRecipientInput {
    pub base_notification_recipient: AgentPubKey,
    pub target_notifier: AgentPubKey,
}
#[hdk_extern]
pub fn remove_notifier_for_notification_recipient(input: RemoveNotifierForNotificationRecipientInput ) -> ExternResult<()> {
    let links = get_links(input.base_notification_recipient.clone(), LinkTypes::NotificationRecipientToNotifiers, None)?;
    
    for link in links {
        if AgentPubKey::from(EntryHash::from(link.target.clone())).eq(&input.target_notifier) {
            delete_link(link.create_link_hash)?;
        }
    }
    
    let links = get_links(input.target_notifier.clone(), LinkTypes::NotifierToNotificationRecipients, None)?;

    for link in links {
        if AgentPubKey::from(EntryHash::from(link.target.clone())).eq(&input.base_notification_recipient) {
            delete_link(link.create_link_hash)?;
        }
    }

    Ok(())        
}