extern crate rocksdb;
extern crate tempdir;

//use rocksdb::rocksdb::Snapshot;
use rocksdb::*;
use std::ops::Deref;
use std::sync::*;
use std::thread;
use tempdir::TempDir;

fn main() {
    let path = TempDir::new("_rust_rocksdb_iteratortest").expect("");

    let k1 = b"k1";
    let k2 = b"k2";
    let k3 = b"k3";
    let k4 = b"k4";
    let v1 = b"v1111";
    let v2 = b"v2222";
    let v3 = b"v3333";
    let v4 = b"v4444";
    let db = DB::open_default(path.path().to_str().unwrap()).unwrap();
    let p = db.put(k1, v1);
    assert!(p.is_ok());
    let p = db.put(k2, v2);
    assert!(p.is_ok());
    let p = db.put(k3, v3);
    assert!(p.is_ok());
    let expected = vec![
        (k1.to_vec(), v1.to_vec()),
        (k2.to_vec(), v2.to_vec()),
        (k3.to_vec(), v3.to_vec()),
    ];

    // iterator
//    let mut iter = db.iter();
//    iter.seek(SeekKey::Start);
//    assert_eq!(iter.collect::<Vec<_>>(), expected);
    let iterator1 = db.iterator(IteratorMode::Start);
    let result=iterator1.collect::<Vec<_>>();

    println!("ok.");
}
