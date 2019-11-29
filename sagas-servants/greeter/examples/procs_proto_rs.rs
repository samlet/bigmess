extern crate protobuf;

use sagas_greeter::*;

//use addressbook::{AddressBook, Person_PhoneType as PhoneType};
use protobuf::{parse_from_reader, ProtobufResult, Message};
use protobuf::error::ProtobufError;
use std::fs::File;
use std::path::Path;

use sagas_greeter::simple::MyObj;
use sagas_greeter::values::*;

pub fn execute<M : Message>(file_path: &str) -> ProtobufResult<(M)> {
    let mut file = File::open(&Path::new(file_path)).map_err(ProtobufError::IoError)?;
    let dt = parse_from_reader::<M>(&mut file)?;
    Ok((dt))
}

fn print(data: &ExternalLinks) {
    for link in data.get_links() {
        println!("{:?}", link);
    }
}

fn main() {
    let pt_data_file="/pi/stack/data/resources/rs_product_type.data";
    match execute::<ExternalLinks>(pt_data_file) {
        val => {
            print(&val.unwrap());
            println!("ok.");
        }
    }
    execute::<ExternalLinks>(pt_data_file)
        .map(|rs| print(&rs));
}
