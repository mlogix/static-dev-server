use std::env;

use std::{convert::Infallible, net::SocketAddr};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use amoeba::handlers::raw;


#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Server started");
    println!("To try this example, open a browser to http://0.0.0.0:{}/", port);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(raw::static_handler))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}