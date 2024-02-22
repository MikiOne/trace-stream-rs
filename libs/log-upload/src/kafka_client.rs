use std::time::Duration;

use rdkafka::ClientConfig;
use rdkafka::error::KafkaResult;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

use common::log::{debug, error, info};
use trace_stream::settings::KafkaConfig;

#[cfg(feature = "kafka")]
pub struct KafkaPub(FutureProducer);

#[cfg(feature = "kafka")]
impl KafkaPub {
    pub fn new(kafka_config: KafkaConfig) -> KafkaResult<Self> {
        // .set("group.id", "consumer_group")
        let config = ClientConfig::new()
            .set("bootstrap.servers", kafka_config.get_broker())
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", kafka_config.get_username())
            .set("sasl.password", kafka_config.get_password());

        Ok(KafkaPub(config.create()?))
    }

    pub async fn produce(&self, topic: &str, message: &str) {
        let record = FutureRecord::to(topic).key("").payload(message);
        let timeout = Timeout::After(Duration::from_secs(3));

        // 发送消息到主题
        let producer: &FutureProducer = &self.0;
        match producer.send(record, timeout).await {
            Ok(delivery_result) => {
                let (partition, offset) = delivery_result;
                debug!("Message sent successfully，partition：{:?}，offset：{:?}", partition, offset);
            }
            Err(e) => {
                error!("Message sending failed：{:?}", e);
            }
        }
    }
}
