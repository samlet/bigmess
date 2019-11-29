extern crate futures;
extern crate grpc;
extern crate sagas_greeter;

use grpc::Client;
use grpc::ClientStub;

use sagas_greeter::helloworld::*;
use sagas_greeter::helloworld_grpc::*;

use std::env;
use std::sync::Arc;

fn main() {
    let name = env::args()
        .nth(1)
        .map(|s| s.to_owned())
        .unwrap_or_else(|| "world".to_owned());

    let client = Arc::new(Client::new_plain("::1", 50051, Default::default()).unwrap());
    let greeter_client = GreeterClient::with_client(client.clone());
    let greeter_client2 = GreeterClient::with_client(client);

    let mut req = HelloRequest::new();
    req.set_name(name);
    let req2 = req.clone();

    let resp = greeter_client.say_hello(grpc::RequestOptions::new(), req);
    let resp2 = greeter_client2.say_hello(grpc::RequestOptions::new(), req2);

    println!("{:?}", resp.wait());
    println!("{:?}", resp2.wait());
}
