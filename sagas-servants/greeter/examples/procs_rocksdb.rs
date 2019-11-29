extern crate rocksdb;
use rocksdb::{DB, DBVector, Error};
use sagas_greeter::simple::MyObj;
use protobuf::*;

fn get_byte_slice<T: AsRef<[u8]>>(source: &'_ T) -> &'_ [u8] {
    source.as_ref()
}

fn put_val(db: &DB){
    let mut po=MyObj::new();
    let name=String::from("tom");
    po.set_name(name);
    po.set_number(18);
    println!("{}, {}", po.name, po.number);

    let serialized = po.write_to_bytes().unwrap();
    db.put(b"pval", serialized);
}

fn get_val(db: &DB, key: &str) -> MyObj{
    let r: Result<Option<DBVector>, Error> = db.get(key);
    let vector = r.unwrap().unwrap();
    let parsed = parse_from_bytes::<MyObj>(&vector).unwrap();
    parsed
}

fn main() {
    let mut db = DB::open_default("./storage").unwrap();
    db.put(b"my key", b"my value");
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    assert!(db.put(b"k1", b"v1111").is_ok());
    let r: Result<Option<DBVector>, Error> = db.get(b"k1");
    let vector = r.unwrap().unwrap();
    assert!(get_byte_slice(&vector) == b"v1111");

    db.delete(b"my key");

    put_val(&db);
    let ret=get_val(&db, "pval");
    println!("get from db: {:?}", ret);
}
