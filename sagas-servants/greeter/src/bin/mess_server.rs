extern crate futures;

extern crate grpc;
extern crate sagas_greeter;
extern crate httpbis;

use std::thread;

use sagas_greeter::helloworld::*;
use sagas_greeter::helloworld_grpc::*;
use grpc::ServerHandlerContext;
use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;

use sagas_greeter::mess_grpc::*;
use sagas_greeter::mess::*;
//use std::error::Error;

use whatlang::{detect, Lang, Script};

struct GreeterImpl {
    instance: u32,
}

impl Greeter for GreeterImpl {
    fn say_hello(&self, _: ServerHandlerContext, req: ServerRequestSingle<HelloRequest>, resp: ServerResponseUnarySink<HelloReply>) -> grpc::Result<()> {
        let mut r = HelloReply::new();
        let name = if req.message.get_name().is_empty() {
            "world"
        } else {
            req.message.get_name()
        };
        println!("instance {}, greeting request from {}", self.instance, name);
        r.set_message(format!("Hello {}", name));
        resp.finish(r)
    }
}

struct MessServiceImpl {
    instance: u32,
}

impl MessService for MessServiceImpl {
    // fn say_hello(&self, o: ServerHandlerContext, req: ServerRequestSingle<MessString>, resp: ServerResponseUnarySink<TextLangInfo>) -> Result<(), Error> {
    fn say_hello(&self, o: ServerHandlerContext, req: ServerRequestSingle<MessString>, resp: ServerResponseUnarySink<TextLangInfo>) -> grpc::Result<()> {
        let text=req.message.get_body();
        println!(".. receive {}", text);
        let info = detect(text).unwrap();
        println!("lang: {} - {}", info.lang(), info.lang().code());
        let mut r= TextLangInfo::new();
        r.set_lang(info.lang().to_string());
        r.set_code(info.lang().code().to_string());
        r.set_confidence(info.confidence() as f32);
        r.set_reliable(info.is_reliable());
        resp.finish(r)
    }
}

fn main() {
    let mut conf = httpbis::ServerConf::default();
    conf.reuse_port = Some(true);

    let mut server1 = grpc::ServerBuilder::new_plain();
    server1.http.conf = conf.clone();
    server1.http.set_port(50051);
    server1.add_service(GreeterServer::new_service_def(GreeterImpl { instance: 1 }));
    server1.add_service(MessServiceServer::new_service_def(MessServiceImpl { instance: 1 }));
    let _server1 = server1.build().expect("server 1");

    let mut server2 = grpc::ServerBuilder::new_plain();
    server2.http.conf = conf.clone();
    server2.http.set_port(50052);
    // server2.add_service(GreeterServer::new_service_def(GreeterImpl { instance: 2 }));
    server2.add_service(MessServiceServer::new_service_def(MessServiceImpl { instance: 2 }));
    let _server2 = server2.build().expect("server 1");

    println!(".. serving");

    loop {
        thread::park();
    }
}
