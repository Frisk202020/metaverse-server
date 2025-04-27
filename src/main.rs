use std::{fs::OpenOptions, net::SocketAddr, sync::Arc, io::Write};
use axum::{routing, Router};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::FmtSubscriber;
use util::*;
use anyhow::Result;
use tower::ServiceBuilder;

mod util;
mod file;
mod directory;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let state = Arc::new(AppState {});
    let mut socket = "0.0.0.0:10000".parse::<SocketAddr>()?; // put port 0 for random port
    let listener = tokio::net::TcpListener::bind(socket).await?;
    socket = listener.local_addr()?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let traced_service = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .service(ServeDir::new("data/341.721.107.002/clovernet"));

    let router = Router::new()
        .route("/download/{ip}", routing::get(download))
        .route("/read", routing::get(read))
        .route("/get/{computer}/{*file}", routing::get(get_file))
        .nest_service("/get_html/341.721.107.002/CloverNet.html", traced_service)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    tokio::spawn(async move {
        if let Err(error) = axum::serve(listener, router.into_make_service()).await {
            println!("server error: {}", error);
        }
    });

    let mut file = OpenOptions::new()
        .append(false)
        .create(false)
        .write(true)
        .open("service.json")?;

    writeln!(file, "{}{}{}{}{}{}{}{}{}{}{}{}{}", "{", '"', "serve", '"', ":true, ", '"', "address", '"', ':', '"', socket.to_string(), '"', '}')?;
    
    println!("Server launched at address: {socket}");
    let _ = tokio::signal::ctrl_c().await;
    println!("Shutting down server");
    Ok(())
}
