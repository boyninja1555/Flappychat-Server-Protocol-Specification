# Connection handshake - [Handshakes and Authentication](index.md) - [Flappychat Server Protocol Specification](../index.md)

This handshake is currently a placeholder in the example server, but you may use it if you don't want to go directly to authentication. For example, to implement IP ban checks.

Your server should be configured to accept the following parameters in the [socket.io](https://socket.io) connection:

```typescript
```

In a Rust Flappychat Server, the code for handling messages should look something like this:

```rust
// Creates the socket.io server instance.
let (layer, io) = SocketIo::builder().build_layer();

// Creates a memory-safe version of IO we can use.
let io_clone: SocketIo = io.clone();
io.ns("/", move |socket: SocketRef| {
    // Listens for the client attempting a connection.
    socket.on("try-connect", |socket: SocketRef| async move {
        // FIXME: Directly tries authentication.
        socket.emit("try-auth", &()).ok();
    });

    // ...
});
```
