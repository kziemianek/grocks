use rocksdb::DB;
use tokio::runtime::Handle;
use std::sync::Arc;

#[tokio::main]
pub async fn main() {
    println!("hello");
    let db = Arc::new(DB::open_default("storage_db").unwrap());

    let key = [0u8; 4];
    let value = [2u8; 4];

    println!("First value read: {:?}", db.get(key).unwrap() );
    let handle = Handle::current();

    let cloned_db = db.clone();
    handle.spawn(async move {
        cloned_db.put(key.clone(), value.clone());
        println!("Second value read: {:?}", db.get(key).unwrap() );
    }).await;
}