#![allow(unused_imports)]
extern crate futures;
extern crate grpc;
extern crate sagas_greeter;

use grpc::Client;
use grpc::ClientStub;
use crate::futures::Future;

//use std::env;
use std::sync::Arc;
use sagas_greeter::services_common_grpc::SysInfoClient;
use sagas_greeter::services_common::InfoQuery;
use sagas_greeter::metainfo::{MetaQuery, MetaPayload, MetaEntity};
use protobuf::{parse_from_bytes, text_format};
use sagas_greeter::metainfo::MetaPayloadType::META_ENTITY;

fn main() {
    let client = Arc::new(Client::new_plain("::1", 50051, Default::default()).unwrap());
    let greeter_client = SysInfoClient::with_client(client.clone());

    let mut req = InfoQuery::new();
    req.queryItems.push("default".to_string());

    let resp = greeter_client.get_sys_info(grpc::RequestOptions::new(), req);
    println!("{:?}", resp.wait());

    let mut q = MetaQuery::new();
    q.set_infoType("entity".to_string());
    q.set_uri("Person".to_string());
    let resp = greeter_client.query_meta(grpc::RequestOptions::new(), q);
    let payload = resp.drop_metadata() // Drop response metadata
        .wait()
        .expect("get_payload");

    println!(".. type - {:?}", payload.field_type);
    if payload.field_type == META_ENTITY {
        let ent = parse_from_bytes::<MetaEntity>(&payload.body).unwrap();
        let mut msg:String=String::new();
        text_format::print_to(&ent, &mut msg);
        println!(".. entity info - {}", msg);
    }
}
