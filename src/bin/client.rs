use std::env;
use std::io::{self, Write};
use std::net::UdpSocket;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> std::io::Result<()> {
    // Get the local port from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <local_port>", args[0]);
        return Ok(());
    }
    let local_port = &args[1];

    let socket = UdpSocket::bind(format!("127.0.0.1:{}", local_port))?; // Bind to the specified port
    socket.connect("127.0.0.1:8080")?;

    // Set terminal to raw mode to capture single key presses
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;
    let mut keys = stdin.keys();

    println!("Press any key to send, 'q' to quit:");

    // Channel to communicate between threads
    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();

    // Thread to handle sending key presses
    let send_socket = socket.try_clone()?;
    thread::spawn(move || {
        while let Ok(byte) = rx.recv() {
            send_socket.send(&[byte]).expect("Failed to send data");
        }
    });

    // Thread to handle receiving server responses
    let recv_socket = socket.try_clone()?;
    thread::spawn(move || {
        let mut buf = [0; 1]; // Buffer to receive a single byte
        loop {
            let (amt, _src) = recv_socket
                .recv_from(&mut buf)
                .expect("Failed to receive data");
            if amt > 0 {
                println!("{:?}", buf[0] as char);
            }
        }
    });

    // Main loop to capture key presses
    loop {
        if let Some(Ok(key)) = keys.next() {
            let byte = match key {
                termion::event::Key::Char('q') => break, // Exit on 'q'
                termion::event::Key::Char(c) => c as u8,
                termion::event::Key::Ctrl('c') => break, // Exit on Ctrl+C
                _ => continue,                           // Ignore other keys
            };

            tx.send(byte).expect("Failed to send key press");
        }
    }

    Ok(())
}
