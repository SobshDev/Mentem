mod init;
mod modules;

use axum::Router;

#[tokio::main]
async fn main()
{
    let app = init::init().await;
    serve(app.router, app.port).await;
}

async fn serve(app: Router, port: u16)
{
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}
