use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// NOTE: Ended on this page: https://tokio.rs/tokio/tutorial/channels
#[tokio::main]
async fn main() {
    let binding = TcpListener::bind("127.0.0.1:6379").await;
    let listener = binding.expect("Failed to bind to port");

    let db: Db = Arc::new(Mutex::new(HashMap::<String, Bytes>::new()));

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let db = db.clone();
        println!("GOT CONNECTION");
        tokio::spawn(async move { process(socket, db).await });
    }
}

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
