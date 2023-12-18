use mini_redis::client;
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

use std::time::Duration;
use std::thread;

#[derive(Debug)]
enum Command{
    Get{
        key: String,
        resp_tx: Response<Option<Bytes>>
    },
    Set{
        key: String,
        value: Bytes,
        resp_tx: Response<()>
    }
}

type Response<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main(){
    let (tx, mut rx) = mpsc::channel(32);
    let tx_ = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp_tx} => {
                    let ret = client.get(&key).await;
                    let _ = resp_tx.send(ret);
                },
                Set { key, value, resp_tx} => {
                    let ret = client.set(&key, value).await;
                    let _ = resp_tx.send(ret);
                }
            }
        }
    });

    let task_1 = tokio::spawn(async move {
        use Command::*;
        let (response_tx, response_rx) = oneshot::channel();
        let cmd = Set { key: String::from("foo"), value: "hello".into(), resp_tx: response_tx };
        tx.send(cmd).await.unwrap();
        let response = response_rx.await.unwrap().unwrap();
        println!("Task 1: Received Response: {:?}", response);
    });

    let task_2 = tokio::spawn(async move {
        thread::sleep(Duration::from_millis(1));

        use Command::*;
        let (response_tx, response_rx) = oneshot::channel();
        let cmd = Get { key: String::from("foo"), resp_tx: response_tx };
        tx_.send(cmd).await.unwrap();
        let response = response_rx.await.unwrap().unwrap();
        println!("Task 2: Received Response: {:?}", response);
    });

    task_1.await.unwrap();
    task_2.await.unwrap();
    manager.await.unwrap();


}