use std::io;
use std::net::SocketAddr;
use tokio::net::TcpStream;

use lib::{config::Config, parser::*};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tracing_appender::rolling;

#[tokio::main]
async fn main() -> io::Result<()> {
    let debug_file = rolling::never("./logs", "debug");
    let cfg = Config::from_env_and_args();

    tracing_subscriber::fmt()
        .with_writer(debug_file)
        .compact()
        .with_thread_ids(true)
        .init();

    let (file_map, var_map) = read_xml_test();

    let listener = TcpListener::bind(cfg.ipaddr.to_string() + ":" + &cfg.port.to_string()).await?;

    tracing::info!(
        "server running on {}",
        cfg.ipaddr.to_string() + ":" + &cfg.port.to_string()
    );

    loop {
        let (stream, addr) = listener.accept().await?;
        tracing::info!("new client: {}", addr);
        tokio::spawn(async move {
            tracing::debug!("accepted connection");
            process(stream, addr).await;
        });
    }
}

async fn process(mut stream: TcpStream, _addr: SocketAddr) {
    stream.write(b"connect\n\n").await.expect("msg");
    stream.write(b"events plain all\n\n").await.expect("msg");
    loop {
        let mut buf = vec![0; 10000];

        let n = stream.read(&mut buf[..]).await.unwrap();

        if n == 0 {
            break;
        }

        tracing::info!("\n{}", String::from_utf8_lossy(&buf));

        // println!("{}",String::from_utf8_lossy(& buf));
    }
}
