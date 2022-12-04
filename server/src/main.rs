use clap::Parser;
use tokio::{net::TcpListener, sync::broadcast, io::BufReader};
use std::str;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value_t = 8000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let listener = TcpListener::bind(format!("localhost:{}", args.port))
        .await
        .expect("could not start the listener");
    let (sender, _) = broadcast::channel::<String>(16);

    loop {
        let (mut stream, _addr) = match listener.accept().await {
            Ok((stream, addr)) => (stream, addr),
            Err(e) => {
                println!("could not acquire socket, {:?}", e);
                continue;
            }
        };

        let channel_sender = sender.clone();
        let mut channel_receiver = channel_sender.subscribe();

        tokio::spawn(async move {
            let (socket_reader, socket_writer) = stream.split();

            let mut buf = vec![0u8; 1024];

            loop {
                tokio::select! {
                    _ = socket_reader.readable() => {
                        match socket_reader.try_read(&mut buf) {
                            Ok(n) => {
                                buf.truncate(n);
                            },
                            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                                continue;
                            },
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        };
                        let message = str::from_utf8(&buf).unwrap();
                        channel_sender.send(message.to_string()).unwrap();
                    },
                    msg = channel_receiver.recv() => {
                        let msg = msg.unwrap();
                        let msg_as_bytes = str::as_bytes(&msg);

                        socket_writer.writable().await.unwrap();
                        socket_writer.try_write(msg_as_bytes).unwrap();
                    },
                };
            };
        });
        
    };
}

