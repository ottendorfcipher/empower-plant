use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::KafkaError;
use rdkafka::util::Timeout;

// Function to create and configure the Kafka producer
pub fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        // Additional configuration options can be set here if necessary
        .create()
        .expect("Producer creation error")
}

pub async fn send_message(producer: &FutureProducer, topic: &str, key: &str, message: &str) -> Result<(), KafkaError> {
    let record = FutureRecord::to(topic)
        .key(key)
        .payload(message);

    let timeout = Timeout::After(std::time::Duration::from_secs(10));

    match producer.send(record, timeout).await {
        Ok((_partition, _)) => Ok(()),
        Err((err, _owned_message)) => Err(err),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_message() {
        let producer = create_producer("localhost:9092");
        let result = send_message(&producer, "test", "key", "message").await;
        assert!(result.is_ok());
    }
}
