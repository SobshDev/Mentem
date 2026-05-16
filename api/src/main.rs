mod init;
mod modules;

use std::net::SocketAddr;

use axum::Router;

#[tokio::main]
async fn main()
{
    let app = init::init().await;
    serve(app.router, app.port).await;
}

async fn serve(app: Router, port: u16)
{
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");
    let service = app.into_make_service_with_connect_info::<SocketAddr>();

    tracing::info!("listening on http://{addr}");
    axum::serve(listener, service).await.expect("server error");
}
