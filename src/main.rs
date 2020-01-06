extern crate chrono;

use std::{convert::Infallible, net::SocketAddr};
use std::env;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Server, Request, Body};
use hyper::server::conn::AddrStream;

use amoeba::handlers::raw;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    let port = match env::var("AMOEBA_PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };

    let host = match env::var("AMOEBA_HOST") {
        Ok(p) => p.parse::<String>().unwrap(),
        Err(..) => String::from("0.0.0.0"),
    };

    let address = format!("{}:{}", host, port);

    let addr = match address.as_str().parse::<SocketAddr>() {
        Ok(a) => a,
        Err(..) => SocketAddr::from(([0, 0, 0, 0], port))
    };

    println!("Server started: http://{}:{}/\n\n", host, port);

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |request: Request<Body>| async move {
                let system_time = SystemTime::now();
                let datetime: DateTime<Utc> = system_time.into();

                println!(
                    "{}\t {}\t\t{}:\t{}",
                     datetime.format("%d/%m/%Y %T.%f"),
                     remote_addr,
                     request.method(),
                     request.uri().path()
                );

                raw::static_handler(request)
            }))
        }
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