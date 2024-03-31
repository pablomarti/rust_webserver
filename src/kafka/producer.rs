use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::boxed::Box;
use std::time::Duration;

pub async fn produce(message: &str, topic: &str) -> Result<(i32, i64), Box<dyn std::error::Error>> {
    let mut kafka_config = ClientConfig::new();
    kafka_config.set("bootstrap.servers", "localhost:29092");

    let producer: FutureProducer = kafka_config.create()?;

    let result = producer
        .send(
            FutureRecord::to(topic)
                .payload(message.as_bytes())
                .key("event1"),
                Duration::from_secs(5),
        )
        .await;

    match result {
      Ok((partition, offset)) => Ok((partition, offset)),
      Err((e, _)) => Err(Box::new(e)),
    }
}
