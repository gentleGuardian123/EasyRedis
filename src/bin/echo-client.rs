use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:8001").await?;
    let (mut reader, mut writer) = io::split(socket);

    tokio::spawn(async move {
        writer.write_all(b"This is the first sentence.\r\n").await?;
        writer.flush().await?;
        writer.write_all(b"This is the second sentence.\r\n").await?;
        writer.flush().await?;
        writer.write_all(b"").await?;
        writer.flush().await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 { break; }
        println!("Received message from server: {:?}", String::from_utf8(buf[..n].to_vec()));
    }

    Ok(())

}

