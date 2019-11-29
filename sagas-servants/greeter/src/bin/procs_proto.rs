extern crate sagas_greeter;
extern crate protobuf;

use sagas_greeter::simple::MyObj;
use sagas_greeter::hex_util::encode_hex;
use sagas_greeter::metainfo::*;

use protobuf::*;

fn main() {
    let rust = "Rust";
    println!("Hello, {}!", rust);

    let mut po=MyObj::new();
    let name=String::from("tom");
    po.set_name(name);
    po.set_number(18);
    println!("{}, {}", po.name, po.number);

    let serialized = po.write_to_bytes().unwrap();
    println!("{}: {}", serialized.len(), encode_hex(&serialized));
    let mut is = CodedInputStream::from_bytes(&serialized);
    let parsed = parse_from_bytes::<MyObj>(&serialized).unwrap();
    println!("{}, {}", parsed.name, parsed.number);
    println!("{}", text_format::print_to_string(&parsed));

    let mut q=MetaQuery::new();
    q.set_infoType("entity".to_string());
    q.set_uri("Person".to_string());
    println!("{}", text_format::print_to_string(&q));
}
