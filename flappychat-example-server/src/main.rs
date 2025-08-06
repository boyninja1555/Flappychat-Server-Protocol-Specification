use socketioxide::SocketIo;
use socketioxide::extract::{Data, SocketRef};

#[derive(serde::Serialize)]
struct AuthField {
    id: String,
    required: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthData {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MessageData {
    details: AuthData,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::builder().build_layer();

    let io_clone: SocketIo = io.clone();
    io.ns("/", move |socket: SocketRef| {
        // Handshake shit
        socket.on("try-connect", |socket: SocketRef| async move {
            socket.emit("try-auth", &()).ok();
        });

        socket.on("get-auth-fields", |socket: SocketRef| async move {
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
            socket
                .emit(
                    "auth-fields",
                    &serde_json::json!({
                        "fields": auth_fields,
                    }),
                )
                .ok();
        });

        socket.on(
            "request-auth",
            |socket: SocketRef, Data(data): Data<AuthData>| async move {
                socket.emit("authed", &serde_json::json!(data)).ok();
            },
        );

        // Actual shit
        let io_inner: SocketIo = io_clone.clone();
        socket.on(
            "send-message",
            move |_socket: SocketRef, Data(data): Data<MessageData>| {
                let io_inner: SocketIo = io_inner.clone();
                async move {
                    io_inner.emit("new-message", &serde_json::json!({
                        "username": data.details.username,
                        "message": data.message,
                    })).await.ok();
                }
            },
        );
    });

    let cors: tower_http::cors::CorsLayer = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any);
    let app: axum::Router = axum::Router::new().layer(cors).layer(layer);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
