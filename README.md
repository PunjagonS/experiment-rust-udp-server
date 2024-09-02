# udp-server
Game nes server project

## Running the Server
To run the server, use the following command:

```
cargo run --bin server
```

## Running Multiple Clients
To run multiple clients, each client needs to bind to a different local port. Use the following commands to run clients on different ports:

```
cargo run --bin client 4000
cargo run --bin client 4001
cargo run --bin client 4002
```
Replace `4000`, `4001`, `4002`, etc., with the desired local ports for each client instance.

This will allow you to test server broadcasts with multiple clients.