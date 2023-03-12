use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener, sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe(); 

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut buf_reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                let bytes_read = buf_reader.read_line(&mut line).await.unwrap();

                // client disconnected
                if bytes_read == 0 {
                    break;
                }

                tx.send(line.clone()).unwrap();

                let msg = rx.recv().await.unwrap();

                writer.write_all(msg.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
