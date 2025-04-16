use rocksdb::DB;

pub fn main() {
    println!("hello");
    let db = DB::open_default("storage_db").unwrap();

    let key = [0u8; 4];
    let value = [2u8; 4];

    println!("First value read: {:?}", db.get(key).unwrap() );
    db.put(key.clone(), value.clone());
    println!("Second value read: {:?}", db.get(key).unwrap() );
}