extern crate jieba_rs;
extern crate tantivy;
#[macro_use]
extern crate lazy_static;

extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate grpc_protobuf;
extern crate protobuf;
extern crate tls_api;

pub mod helloworld;
pub mod helloworld_grpc;
//pub mod hello;
//pub mod hello_grpc;

pub mod simple;
pub mod addressbook;
pub mod common_types;
pub mod nlpserv;
pub mod nlpserv_grpc;
pub mod metainfo;

pub mod mess;
pub mod mess_grpc;
pub mod values;
pub mod services_common;
pub mod services_common_grpc;

pub mod hex_util;
pub mod list_people;
pub mod add_person;

pub mod jieba_tokenizer;
pub mod rpc_broker;
