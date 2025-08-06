## Auth fields handshake - [Handshakes and Authentication](index.md)

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
    // ...

    // Listens for client requesting authentication fields (e.g. username, password.)
    socket.on("get-auth-fields", |socket: SocketRef| async move {
        // Creates the list storing the authentication fields.
        let auth_fields: [AuthField; 2] = [
            AuthField {
                id: String::from("username"),
                required: true,
            },
            AuthField {
                id: String::from("password"),
                required: true,
            },
        ];

        // Sends these fields back to the client.
        socket
            .emit(
                "auth-fields",
                &serde_json::json!({
                    "fields": auth_fields,
                }),
            )
            .ok();
    });

    // ...
});
```
