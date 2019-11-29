extern crate rocksdb;
use rocksdb::{DB, WriteBatch};

fn main() {
    // NB: db is automatically freed at end of lifetime
    let mut db = DB::open_default("./storage").unwrap();
    {
        let mut batch = WriteBatch::default(); // WriteBatch and db both have trait Writable
        batch.put(b"my key", b"my value");
        batch.put(b"key2", b"value2");
        batch.put(b"key3", b"value3");
        db.write(batch); // Atomically commits the batch
    }
}
