use tokio::{task, net::{TcpListener, TcpStream}};
use mini_redis::{Connection, Frame};
use bytes::Bytes;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {

    let lstr = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = lstr.accept().await.unwrap();
        let db = db.clone();

        task::spawn(async move {
            process_socket(socket, db).await;
        });
    }
}

async fn process_socket(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // let mut db = HashMap::new();
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("Unimplemented {:?}!", cmd),
        };

        conn.write_frame(&response).await.unwrap();
    }

}
