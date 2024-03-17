// src/consumers/user_events_consumer.rs
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaResult;

// Function to create and configure the Kafka consumer
pub async fn create_and_consume(brokers: &str, group_id: &str) -> KafkaResult<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "true")
        .create()?;
    consumer.subscribe(&["user_created", "user_updated", "user_deleted"])?;

    // Consuming messages
    loop {
        match consumer.recv().await {
            Ok(m) => println!("Received: {:?}", m),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
