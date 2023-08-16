use std::{str::FromStr, sync::Arc};

use anyhow::bail;
use aruna_rust_api::api::{
    notification::services::v2::{
        anouncement_event::EventVariant as AnnEventVariant, event_message::MessageVariant,
        AnouncementEvent, EventVariant, ResourceEvent, UserEvent,
    },
    storage::models::v2::generic_resource,
};
use aruna_rust_api::api::storage::models::v2::User as ApiUser;
use async_nats::jetstream::consumer::DeliverPolicy;
use diesel_ulid::DieselUlid;
use futures::StreamExt;

use crate::{
    database::{
        connection::Database,
        crud::CrudDb,
        dsls::{object_dsl::Object, user_dsl::User},
    },
    notification::natsio_handler::NatsIoHandler,
    utils::grpc_utils::{checksum_resource, checksum_user},
};

use super::cache::Cache;

pub struct NotificationHandler {
    //database: Arc<Database>,
    //cache: Arc<Cache>,
    //natsio_handler: Arc<NatsIoHandler>,
    //stream_consumer: PushConsumer,
}

// Nats.io handler direkt
// Message stream?
impl NotificationHandler {
    ///ToDo: Rust Doc
    pub async fn new(
        database: Arc<Database>,
        cache: Arc<Cache>,
        natsio_handler: Arc<NatsIoHandler>,
    ) -> anyhow::Result<Self> {
        // Create push consumer for all notifications
        let myself_id = DieselUlid::generate(); //ToDo: Replace with instance id?
        let (consumer_id, _) = natsio_handler
            .create_push_consumer(myself_id, "AOS.>".to_string(), DeliverPolicy::All, true)
            .await?;

        // Fetch push consumer from stream
        let push_consumer = natsio_handler
            .get_push_consumer(consumer_id.to_string())
            .await?;

        // Async move the consumer listening
        let mut messages = push_consumer.messages().await?;
        let cache_clone = cache.clone();
        let database_clone = database.clone();
        let _ = tokio::spawn(async move {
            loop {
                if let Some(Ok(nats_message)) = messages.next().await {
                    log::debug!("got message {:?}", nats_message);

                    let msg_variant = match serde_json::from_slice(
                        nats_message.message.payload.to_vec().as_slice(),
                    ) {
                        Ok(variant) => variant,
                        Err(err) => {
                            return Err::<MessageVariant, anyhow::Error>(anyhow::anyhow!(err))
                        }
                    };

                    // Update cache
                    NotificationHandler::update_server_cache(
                        msg_variant,
                        cache_clone.clone(),
                        database_clone.clone(),
                    )
                    .await?;
                }
            }
        })
        .await?;

        // Return ... something
        Ok(NotificationHandler {})
    }

    ///ToDo: Rust Doc
    async fn update_server_cache(
        message: MessageVariant,
        cache: Arc<Cache>,
        database: Arc<Database>,
    ) -> anyhow::Result<()> {
        match message {
            MessageVariant::ResourceEvent(event) => {
                process_resource_event(event, cache, database).await?
            }
            MessageVariant::UserEvent(event) => {
                process_user_event(event, cache, database).await?;
            }

            MessageVariant::AnnouncementEvent(event) => {
                process_announcement_event(event)?;
            }
        }

        Ok(())
    }
}

/* -------------- Helper -------------- */
async fn process_resource_event(
    resource_event: ResourceEvent,
    cache: Arc<Cache>,
    database: Arc<Database>,
) -> anyhow::Result<()> {
    if let Some(resource) = resource_event.resource {
        // Extract resource id
        let resource_ulid = DieselUlid::from_str(&resource.resource_id)?;

        // Process cache
        if let Some(variant) = EventVariant::from_i32(resource_event.event_variant) {
            match variant {
                EventVariant::Unspecified => bail!("Unspecified event variant not allowed"),
                EventVariant::Created | EventVariant::Updated => {
                    if let Some(object_plus) = cache.get_object(&resource_ulid) {
                        // Convert to proto and compare checksum
                        let proto_resource: generic_resource::Resource =
                            object_plus.clone().try_into()?;
                        let proto_checksum = checksum_resource(proto_resource)?;

                        if !(proto_checksum == resource.checksum) {
                            // Things that should not happen with a 'Created' event ...
                            cache.update_object(&resource_ulid, object_plus);
                        }
                    } else {
                        // Fetch object with relations from database and put into cache
                        let client = database.get_client().await?;
                        let object_plus =
                            Object::get_object_with_relations(&resource_ulid, &client).await?;

                        cache.object_cache.insert(resource_ulid, object_plus);
                    }
                }
                EventVariant::Available => {
                    todo!("Ignore or set resource available in cache")
                }
                EventVariant::Deleted => {
                    // Just delete resource from cache
                    //  Or set status to deleted?
                }
            }
        } else {
            // Return error if variant is None
            bail!("Resource event variant missing")
        }
    } else {
        // Return error
        bail!("Resource event resource missing")
    }

    Ok(())
}

async fn process_user_event(
    user_event: UserEvent,
    cache: Arc<Cache>,
    database: Arc<Database>,
) -> anyhow::Result<()> {
    // Extract user id
    let user_ulid = DieselUlid::from_str(&user_event.user_id)?;

    // Process cache
    if let Some(variant) = EventVariant::from_i32(user_event.event_variant) {
        match variant {
            EventVariant::Unspecified => bail!("Unspecified user event variant not allowed"),
            EventVariant::Created => {
                // Check if user already exists
                if let Some(user) = cache.get_user(&user_ulid) {
                    // Convert to proto and compare checksum
                    let proto_user = ApiUser::from(user.clone());
                    let proto_checksum = checksum_user(&proto_user)?;

                    if !(proto_checksum == user_event.checksum) {
                        cache.update_user(&user_ulid, user);
                    }
                } else {
                    // Fetch user from database and add to cache
                    let client = database.get_client().await?;
                    if let Some(user) = User::get(user_ulid, &client).await? {
                        cache.add_user(user_ulid, user)
                    } else {
                        bail!("User does not exist")
                    }
                }
            }
            EventVariant::Available => todo!(),
            EventVariant::Updated => todo!(),
            EventVariant::Deleted => todo!(),
        }
    } else {
        // Return error if variant is None
        bail!("User event variant missing")
    }

    Ok(())
}

fn process_announcement_event(announcement_event: AnouncementEvent) -> anyhow::Result<()> {
    if let Some(variant) = announcement_event.event_variant {
        match variant {
            AnnEventVariant::NewDataProxyId(_) => unimplemented!("?"),
            AnnEventVariant::RemoveDataProxyId(_) => unimplemented!("?"),
            AnnEventVariant::UpdateDataProxyId(_) => unimplemented!("?"),
            AnnEventVariant::NewPubkey(_) => unimplemented!("Refresh pubkey cache"),
            AnnEventVariant::RemovePubkey(_) => unimplemented!("Refresh pubkey cache"),
            AnnEventVariant::Downtime(_) => {
                unimplemented!("Prepare for downtime. Degradation or something ...")
            }
            AnnEventVariant::Version(_) => unimplemented!("Ignore"),
        }
    } else {
        // Return error if variant is None
        bail!("Announcement event variant missing")
    }
}
