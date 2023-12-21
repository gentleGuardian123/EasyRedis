use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("examples/hello-redis.rs").await?;
    let mut buffer = [0;10];

    let n = f.read(&mut buffer[..]).await?;
    println!("{:?}", &buffer[..n]);
    Ok(())
}
