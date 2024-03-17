use rdkafka::config::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::{Message, BorrowedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::KafkaResult;
use futures::stream::StreamExt;

pub fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .set("acks", "all") // Ensure all replicas acknowledge
        .create()
        .expect("Producer creation error")
}

pub fn create_consumer(brokers: &str, group_id: &str, topics: &[&str]) -> KafkaResult<StreamConsumer> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false") // Manual offset commit
        .create()?;

    consumer.subscribe(topics)?;
    Ok(consumer)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let producer = create_producer("localhost:9092");

    let consumer = create_consumer("localhost:9092", "user_management_group", &["user_created", "user_updated"])?;

    let record = FutureRecord::to("user_created")
        .payload("User creation event payload")
        .key("user_key");
    let produce_future = producer.send(record, None);
    if let Ok(delivery) = produce_future.await {
        println!("Sent: {:?}", delivery);
    }

    let mut message_stream = Box::pin(consumer.stream());
    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => process_message(&m),
            Err(e) => println!("Kafka error: {:?}", e),
        }
    }

    Ok(())
}

fn process_message(message: &BorrowedMessage) {
    match message.payload_view::<str>() {
        Some(Ok(payload)) => println!("Received message: {}", payload),
        Some(Err(_)) => println!("Payload is not a valid UTF-8 string"),
        None => println!("Received message with empty payload"),
    }
}

