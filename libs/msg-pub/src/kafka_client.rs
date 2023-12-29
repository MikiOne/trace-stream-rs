use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use common::settings::KafkaConfig;


#[cfg(feature = "kafka")]
pub struct Producer(FutureProducer);

#[cfg(feature = "kafka")]
impl Producer {
    pub fn new(kafka_config: KafkaConfig) -> Self {
        let broker = kafka_config.get_broker();
        let mut client = ClientConfig::new();
        let producer_config = client.set("bootstrap.servers", broker)
            // .set("group.id", "consumer_group")
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", kafka_config.get_username())
            .set("sasl.password", kafka_config.get_password())
            ;

        let producer: FutureProducer = producer_config.create().expect("创建生产者失败");
        Producer(producer)
    }

    pub async fn produce(&self, topic: &str, message: &str) {
        // 创建生产者
        let producer: &FutureProducer = &self.0;
        let record = FutureRecord::to(topic).key("").payload(message);

        let timeout = Timeout::After(Duration::from_secs(3));
        // 发送消息到主题
        match producer.send(record, timeout).await {
            Ok(delivery_result) => {
                let (partition, offset) = delivery_result;
                println!("消息发送成功，分区：{}，偏移量：{}", partition, offset);
            }
            Err(e) => {
                eprintln!("消息发送失败：{:?}", e);
            }
        }
    }
}
