// server/kafka_integration_tests.rs

mod config;
mod consumers;
mod producers;

use tokio::time::{timeout, Duration};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

// Utility function for setting up a test consumer that listens for a specific message
async fn setup_test_consumer(expected_message: Arc<Mutex<Option<String>>>) {
    let brokers = "localhost:9092";
    let group_id = "test_group";
    let topics = ["user_created"];

    let consumer = create_consumer(brokers, group_id, &topics).expect("Failed to create consumer");

    tokio::spawn(async move {
        for result in consumer.iter().take(10) {
            if let Ok(message) = result {
                if let Some(payload) = message.payload_view::<str>().unwrap_or(None) {
                    let mut expected_msg = expected_message.lock().await;
                    *expected_msg = Some(payload.to_owned().to_string());
                    break;
                }
            }
        }
    });
}

#[tokio::test]
async fn test_user_created_event() {
    let expected_message = Arc::new(Mutex::new(None));
    setup_test_consumer(expected_message.clone()).await;

    let brokers = "localhost:9092";
    let producer = create_producer(brokers);
    let topic = "user_created";
    let user_data = json!({
        "id": "user123",
        "email": "test@example.com",
        "timestamp": "2023-04-01T12:00:00Z",
    }).to_string();

    // Send a test message
    send_message(&producer, topic, "user123", &user_data).await;

    // Wait for the consumer to receive the message or timeout after 5 seconds
    let received_message = timeout(Duration::from_secs(5), async {
        let mut msg = None;
        while msg.is_none() {
            let lock = expected_message.lock().await;
            msg = lock.clone();
        }
        msg
    }).await.expect("Timeout waiting for message").expect("Did not receive message");

    // Assert that the received message matches the expected message
    assert_eq!(received_message, Some(user_data));
}
