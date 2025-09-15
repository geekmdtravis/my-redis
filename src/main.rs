use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

// NOTE: Ended on this page: https://tokio.rs/tokio/tutorial/spawning

#[tokio::main]
async fn main() {
    let response = TcpListener::bind("127.0.0.1:6379").await;
    let listener = response.expect("Failed to bind to port");

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        tokio::spawn(async move { process(socket).await });
    }
}

async fn process(socket: TcpStream) {
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
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
