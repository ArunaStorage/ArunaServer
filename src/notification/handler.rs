
use aruna_rust_api::api::notification::services::v2::{
    anouncement_event::EventVariant as AnnouncementVariant, event_message::MessageVariant, Reply,
};
use async_nats::jetstream::consumer::{DeliverPolicy, Config};
use async_nats::jetstream::Message;
use async_trait::async_trait;
use diesel_ulid::DieselUlid;

use crate::database::enums::ObjectType;

// Internal enum which provides info for consumer creation
pub enum EventType {
    Resource((DieselUlid, ObjectType, bool)),
    User(DieselUlid),
    Announcement(AnnouncementVariant),
    All,
}

// An Event handler is the main connection of the underlaying event message system like Nats.io
#[async_trait]
pub trait EventHandler {
    // Registers/Publishs an event into the event message system
    async fn register_event(
        &self,
        message_variant: MessageVariant,
    ) -> anyhow::Result<()>;

    // Creates an event consumer.
    // An event consumer is a entity of the underlaying event streaming system can be used
    // to load balance a set of incoming messages based on an individual query across multiple
    // clients.
    // This corresponds to a consumer in Nats.io Jetstream https://docs.nats.io/nats-concepts/jetstream
    async fn create_event_consumer(
        &self,
        event_type: EventType,
        delivery_policy: DeliverPolicy,
    ) -> anyhow::Result<(DieselUlid, Config)>;

    /// Fetch messages from the event system concerning the event consumer.
    async fn get_event_consumer_messages(
        &self,
        event_consumer_id: String,
        batch_size: u32,
    ) -> anyhow::Result<Vec<Message>>;

    // Acknowledge messages as read so the will not be redelivered by consecutive message fetches.
    async fn acknowledge_from_reply(&self, replies: Vec<Reply>) -> anyhow::Result<()>;

    // Creates an event stream handler depending on the underlaying system
    // The handler is connected to a stream group to load-balance messages... !?
    async fn create_event_stream_handler(
        &self,
        event_consumer_id: String,
    ) -> anyhow::Result<Box<dyn EventStreamHandler + Send + Sync>>;

    // Deletes a stream group which is represented through a Nats.io Jetstream consumer.
    async fn delete_event_consumer(
        &self,
        event_consumer_id: String,
    ) -> anyhow::Result<()>;
}

// An EventStreamHandler handles the message stream based on StreamGroups/Consumers
#[async_trait]
pub trait EventStreamHandler {
    // Gets a batch of messages from the underlaying event system
    // This call expected to return after a certain timeout even if no messages are available
    // This is currently specific to nats.io and needs to be generalized
    // TODO: Generalize
    async fn get_event_consumer_messages(
        &self,
    ) -> anyhow::Result<Vec<Message>>;
}
