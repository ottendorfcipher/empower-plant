// src/main.rs
mod config;
mod consumers;
mod producers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let brokers = "localhost:9092";
    let group_id = "user_management_group";

    let producer = config::kafka::create_producer(brokers);
    let user_data = "{\"id\": \"user123\", \"email\": \"user@example.com\"}";

    // Sending a message using the producer
    producers::user_events_producer::send_message(&producer, "user_created", "user123", user_data).await?;

    // Starting the consumer in a new async task
    let consumer_task = tokio::spawn(async move {
        consumers::user_events_consumer::create_and_consume(brokers, group_id).await.expect("Consumer error");
    });

    // A simple way to keep the main task alive to listen for consumer messages
    // Replace with your actual application logic or signal handling for graceful shutdown
    consumer_task.await?;

    Ok(())
}
