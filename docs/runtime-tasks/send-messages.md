# Send Messages - [Runtime Tasks](index.md) - [Flappychat Server Protocol Specification](../index.md)

Because the authentication steps only exist to check account validity prior to connection, the account info must be supplied every message. This also ensures an account still exists when an update is required.

Your server should be configured to accept the following parameters in the [socket.io](https://socket.io) connection:

```typescript
data: {
    details: {
        username: string,
        password: string,
    },
    message: string,
},
```

In a Rust Flappychat Server, the code for handling messages should look something like this:

```rust
// Creates the socket.io server instance.
let (layer, io) = SocketIo::builder().build_layer();

// Creates a memory-safe version of IO we can use.
let io_clone: SocketIo = io.clone();
io.ns("/", move |socket: SocketRef| {

    // ...

    // Creates a memory-safe version of IO we can use inside this scope.
    let io_inner: SocketIo = io_clone.clone();

    // Creates the send-message listener.
    socket.on(
        "send-message",

        // Captures the message data
        move |_socket: SocketRef, Data(data): Data<MessageData>| {
            // Creates a newer memory-safe version of IO we can use.
            let io_inner: SocketIo = io_inner.clone();
            async move {
                // Broadcasts the message to every user (do not forget to omit the password!)
                // FIXME: Make it actually verify user details.
                io_inner.emit("new-message", &serde_json::json!({
                    "username": data.details.username,
                    "message": data.message,
                })).await.ok();
            }
        },
    );

    // ...
});
```
