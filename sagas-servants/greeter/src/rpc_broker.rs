#![allow(unused_imports)]
use amiquip::{
    AmqpProperties, Channel, Connection, Consumer, ConsumerMessage, ConsumerOptions, Exchange,
    Publish, Queue, QueueDeclareOptions, Result,
};
use uuid::Uuid;
use protobuf::*;

pub struct MetaRpcClient<'a> {
    exchange: Exchange<'a>,
    queue: Queue<'a>,
    consumer: Consumer<'a>,
    queue_name: String,
}

impl<'a> MetaRpcClient<'a> {
    pub fn new(channel: &Channel, queue_name: String) -> Result<MetaRpcClient> {
        let exchange = Exchange::direct(&channel);

        let queue = channel.queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                ..QueueDeclareOptions::default()
            },
        )?;
        let consumer = queue.consume(ConsumerOptions {
            no_ack: true,
            ..ConsumerOptions::default()
        })?;

        Ok(MetaRpcClient {
            exchange,
            queue,
            consumer,
            queue_name,
        })
    }

    pub fn call(&self, n: &[u8]) -> Result<Option<Vec<u8>>> {
        let correlation_id = format!("{}", Uuid::new_v4());
        self.exchange.publish(Publish::with_properties(
            //format!("{}", n).as_bytes(),
            n,
            self.queue_name.clone(),
            AmqpProperties::default()
                .with_reply_to(self.queue.name().to_string())
                .with_correlation_id(correlation_id.clone()),
        ))?;
        for message in self.consumer.receiver().iter() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    if delivery.properties.correlation_id().as_ref() == Some(&correlation_id) {
                        // return Ok(String::from_utf8_lossy(&delivery.body).into());
                        println!(".. get some response");
                        // let parsed = parse_from_bytes::<MetaEntity>(&delivery.body).unwrap();
                        // println!(".. got - {}", text_format::print_to_string(&parsed));
                        return Ok(Some(delivery.body));
                    }
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        // This should really be an Err(..), but we don't want to go through the trouble
        // of defining a new error type for this example.
        // Ok("ERROR: server failed to respond to RPC call".to_string())
        Ok(None)
    }
}
