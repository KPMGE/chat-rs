
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _) = listener.accept().await.unwrap();
    let mut buffer = [0u8; 1024];
    let bytes_read = socket.read(&mut buffer).await.unwrap();

    println!("messge received: ");
    for c in 0..bytes_read {
        print!("{}", buffer[c] as char);
    }

    socket.write_all(&buffer[..bytes_read]).await.unwrap();
}
