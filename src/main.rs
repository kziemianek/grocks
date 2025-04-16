use rocksdb::DB;
use tokio::runtime::Handle;
use std::sync::Arc;
use jsonrpsee::server::{RpcModule, Server};
use std::net::SocketAddr;

#[tokio::main]
pub async fn main() {
    println!("hello");
    let db = Arc::new(DB::open_default("storage_db").unwrap());

    let key = [0u8; 4];
    let value = [2u8; 4];

    println!("First value read: {:?}", db.get(key).unwrap() );
    let handle = Handle::current();

    let cloned_db = db.clone();


	let server = Server::builder().build("0.0.0.0:2000".parse::<SocketAddr>().unwrap()).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("say_hello", move |_, _, _| {
        cloned_db.put(key, value);
    }).unwrap();


    let handle = server.start(module);

    tokio::spawn(handle.stopped()).await;
}


//     handle.spawn(async move {
//         cloned_db.put(key.clone(), value.clone());
//         println!("Second value read: {:?}", db.get(key).unwrap() );
//     }).await;
// }