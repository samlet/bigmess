#![allow(unused_imports)]
extern crate sagas_greeter;

use amiquip::{
    AmqpProperties, Channel, Connection, Consumer, ConsumerMessage, ConsumerOptions, Exchange,
    Publish, Queue, QueueDeclareOptions, Result,
};
use uuid::Uuid;
use protobuf::*;
use sagas_greeter::rpc_broker::*;

fn main() -> Result<()> {
    use sagas_greeter::metainfo::*;
    use sagas_greeter::metainfo::MetaPayloadType::*;

    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    let rpc_client = MetaRpcClient::new(&channel, "meta_queue".to_string())?;

    println!("Requesting ...");

    let mut q=MetaQuery::new();
    q.set_infoType("entity".to_string());
    q.set_uri("Person".to_string());
    // println!("{}", text_format::print_to_string(&q));
    let serialized = q.write_to_bytes().unwrap();
    let result = rpc_client.call(&serialized)?;
    match result {
        Some(v) => {
            let parsed = parse_from_bytes::<MetaPayload>(&v).unwrap();
            println!(".. type - {:?}", parsed.field_type);
            if parsed.field_type==META_ENTITY {
                let ent=parse_from_bytes::<MetaEntity>(&parsed.body).unwrap();
                println!(".. entity info - {}", text_format::print_to_string(&ent));
            }
        }
        _ => {}
    }

    connection.close()
}
