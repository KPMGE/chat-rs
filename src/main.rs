use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener, sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe(); 

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut buf_reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = buf_reader.read_line(&mut line) => {
                        // client disconnected
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                    },
                    result = rx.recv() => {
                        let (msg, new_addr) = result.unwrap();

                        // if the client is not the sender
                        if addr != new_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                            line.clear();
                        }
                    }
                }
            }
        });
    }
}
