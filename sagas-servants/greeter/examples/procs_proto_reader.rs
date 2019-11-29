extern crate protobuf;

use sagas_greeter::*;

//use addressbook::{AddressBook, Person_PhoneType as PhoneType};
use protobuf::{parse_from_reader, ProtobufResult};
use protobuf::error::ProtobufError;
use std::fs::File;
use std::path::Path;

use sagas_greeter::simple::MyObj;

pub fn execute(file_path: &str) -> ProtobufResult<()> {
    let mut file = File::open(&Path::new(file_path)).map_err(ProtobufError::IoError)?;
    let address = parse_from_reader::<MyObj>(&mut file)?;
    print(&address);
    Ok(())
}
fn print(data: &MyObj) {
    println!("{:?}", data);
}

fn main() {
    match execute("/pi/stack/data/resources/tests_obj.data") {
        _ => {
            println!("ok.");
        }
    }
}
