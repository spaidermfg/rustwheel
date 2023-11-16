use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::accept;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6767").await?;
    println!("websocket server is running...");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream))
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    if let Ok(upgrade) = accept(stream).await {
        let (mut tx, mut rx) = upgrade.split();

        while let Some(Ok(msg)) = rx.next().await {
            if msg.is_text() || msg.is_binary() {
                if let Err(e) = tx.send(msg).await {
                    println!("Error sending message: {:?}", e);
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn working() {
        main()
    }
}
