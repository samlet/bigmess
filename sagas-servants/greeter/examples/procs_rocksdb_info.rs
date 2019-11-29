extern crate rocksdb;
extern crate protobuf;

mod util;

use rocksdb::{ColumnFamilyDescriptor, MergeOperands, Options, DB, DBVector, Error};
//use util::DBPath;
use sagas_greeter::simple::MyObj;
use protobuf::*;

fn get_val(db: &DB, key: &str) -> MyObj{
    let r: Result<Option<DBVector>, Error> = db.get(key);
    let vector = r.unwrap().unwrap();
    let parsed = parse_from_bytes::<MyObj>(&vector).unwrap();
    parsed
}

fn get_cf_val(db: &DB, cf_name:&str, key: &str) -> MyObj{
    let cf1 = db.cf_handle(cf_name).unwrap();

    let r: Result<Option<DBVector>, Error> = db.get_cf(cf1, key);
    let vector = r.unwrap().unwrap();
    let parsed = parse_from_bytes::<MyObj>(&vector).unwrap();
    parsed
}

fn get_cf(){
    let mut opts = Options::default();
    let mut db=  DB::open_cf(&opts, "/pi/stack/db/sys.db", &["default", "property", "value"]);
    match db {
        Ok(_db) => {
            println!("successfully opened db with column family");
            let ret=get_cf_val(&_db, "value", "key");
            println!("get from db: {:?}", ret);
        },
        Err(e) => panic!("failed to open db with column family: {}", e),
    }
}

fn main() {
    // let mut db = DB::open_default("/pi/stack/db/sys.db").unwrap();
    let opts = Options::default();
    let vec = DB::list_cf(&opts, "/pi/stack/db/sys.db");
    match vec {
        Ok(vec) => println!("{:?}", vec),
        Err(e) => panic!("failed to list column family: {}", e),
    }

    get_cf();
}

/*
python side:

```python
from simple_pb2 import MyObj, Foo
obj=MyObj(name='hello')
print(obj)

from sagas.storage.data_space import DataSpace, sys_db
new_cf_column_family = sys_db.db.get_column_family(b'value')

sys_db.db.put( (new_cf_column_family, b'key'), obj.SerializeToString() )
val= sys_db.db.get( (new_cf_column_family, b'key') )
obj=MyObj()
obj.ParseFromString(val)
print(obj)
```
*/
