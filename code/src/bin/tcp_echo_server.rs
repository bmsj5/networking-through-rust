//! TCP echo server — sockets chapter demo.
//!
//! Run: cargo run --bin tcp-echo-server
//! Test: nc 127.0.0.1 7878
//! CI/make: cargo run --bin tcp-echo-server -- --once

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let once = std::env::args().any(|a| a == "--once");
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("echo server on 127.0.0.1:7878 (Ctrl-C to stop)");

    if once {
        // Self-test: connect, send, verify echo, exit.
        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut buf = vec![0u8; 1024];
            let n = socket.read(&mut buf).await.expect("read");
            socket.write_all(&buf[..n]).await.expect("write");
        });
        let mut client = tokio::net::TcpStream::connect("127.0.0.1:7878").await?;
        client.write_all(b"ping").await?;
        let mut got = [0u8; 4];
        client.read_exact(&mut got).await?;
        assert_eq!(&got, b"ping");
        handle.await.ok();
        println!("self-test ok: echoed \"ping\"");
        return Ok(());
    }

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("connection from {addr}");
        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }
}
