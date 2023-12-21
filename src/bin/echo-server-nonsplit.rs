use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8001").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,                                             // The socket has been closed.
                    Ok(n) => {                                            // Read `n` bytes from socket.
                        println!("Received message from client: {:?}", String::from_utf8(buf[..n].to_vec()));
                        if socket.write_all(&mut buf[..n]).await.is_err() {
                            eprintln!("Failed to copy!");
                        }
                    },
                    Err(_) => return,                                            // Unexpected error, do nothing here.
                }
            }
        });

    }
}
