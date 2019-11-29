extern crate rocksdb;
pub mod util;

use rocksdb::{ColumnFamilyDescriptor, MergeOperands, Options, DB};
use util::DBPath;

fn test_provided_merge(
    _: &[u8],
    existing_val: Option<&[u8]>,
    operands: &mut MergeOperands,
) -> Option<Vec<u8>> {
    let nops = operands.size_hint().0;
    let mut result: Vec<u8> = Vec::with_capacity(nops);
    if let Some(v) = existing_val {
        for e in v {
            result.push(*e);
        }
    }
    for op in operands {
        for e in op {
            result.push(*e);
        }
    }
    Some(result)
}

fn main() {
    let n = DBPath::new("_rust_rocksdb_cftest");
    println!("create db in {:?}", n.path);

    // should be able to create column families
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_merge_operator("test operator", test_provided_merge, None);
        let db = DB::open(&opts, &n).unwrap();
        let opts = Options::default();
        match db.create_cf("cf1", &opts) {
            Ok(_db) => println!("cf1 created successfully"),
            Err(e) => {
                panic!("could not create column family: {}", e);
            }
        }
    }
}
