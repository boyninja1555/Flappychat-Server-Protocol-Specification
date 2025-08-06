# Flappychat Server Protocol Specification

If you are developing a Flappychat server for an unsupported language, this documentation will be handy. Inside this repository also includes a working version written in Rust. The following documentation applies to **all Flappychat servers**, not just the example.

Every request and response from and to the Flappychat server is done over [socket.io](https://socket.io), which improves upon WebSockets. Authentication is spread out to more dynamically accept custom systems besides username and password.

## Table of Contents

- [Handshakes and authentication](handshakes-and-authentication/index.md)
    - [Connection handshake](handshakes-and-authentication/connection.md)
    - [Auth fields handshake](handshakes-and-authentication/get-auth-fields.md)
    - [Auth request](handshakes-and-authentication/auth-request.md)
- [Runtime tasks](runtime-tasks/index.md)
    - [Send messages](runtime-tasks/send-messages.md)

## *(Rust)* Crates Used in Example Server

| Crate Name   | Crate Version | Crate Features |
| :----------- | :-----------: | -------------: |
| axum         | 0.8.4         | Empty          |
| serde        | 1.0.219       | Empty          |
| serde_json   | 1.0.142       | Empty          |
| socketioxide | 0.17.2        | Empty          |
| tokio        | 1.47.1        | Full           |
| tower-http   | 0.6.6         | CORS           |

**Rust Toolchain:** Nightly
