## Auth request - [Handshakes and Authentication](index.md)

This handshake is currently a placeholder in the example server, but you may use it if you don't want to go directly to authentication. For example, to implement IP ban checks.

Your server should be configured to accept the following parameters in the [socket.io](https://socket.io) connection:

```typescript
data: {
    username: String,
    password: String,
}
```

In a Rust Flappychat Server, the code for handling messages should look something like this:

```rust
// Creates the socket.io server instance.
let (layer, io) = SocketIo::builder().build_layer();

// Creates a memory-safe version of IO we can use.
let io_clone: SocketIo = io.clone();
io.ns("/", move |socket: SocketRef| {
    // ...

    // Listens for client requesting authentication.
    let io_inner: SocketIo = io_clone.clone();
    socket.on(
        "request-auth",
        |socket: SocketRef, Data(data): Data<AuthData>| async move {
            let io_inner: SocketIo = io_inner.clone();
            let server_username: String = "Server".to_string();

            // Disallows use of the server's username.
            if data.username.trim().to_lowercase() == server_username {
                return;
            }

            // FIXME: Use actual auth.
            socket.emit("authed", &serde_json::json!(data)).ok();

            // Broadcasts the user's join message.
            io_inner.emit("new-user", &serde_json::json!({
                "server_username": server_username,
                "join_message": format!("{} joined the chat!", data.username.trim()),
            })).await.ok();
        },
    );

    // ...
});
```
