use std::collections::HashSet;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

async fn run_udp_server() -> std::io::Result<()> {
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:8080").await?);
    println!("UDP server listening on 127.0.0.1:8080");

    let mut buf = [0; 1024];
    let clients = Arc::new(Mutex::new(HashSet::new()));

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}", len, addr);

        // Print the received bytes
        println!("Received: {:?}", &buf[..len]);

        // Add the client address to the set
        {
            let mut clients = clients.lock().await;
            clients.insert(addr);
        }

        // Echo the received bytes back to all clients
        let clients = clients.clone();
        let socket = socket.clone();
        let buf = buf[..len].to_vec();
        tokio::spawn(async move {
            let clients = clients.lock().await;
            for &client in clients.iter() {
                if let Err(e) = socket.send_to(&buf, &client).await {
                    eprintln!("Failed to send to {}: {}", client, e);
                }
            }
        });
    }
}

fn main() -> std::io::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(run_udp_server())
}
