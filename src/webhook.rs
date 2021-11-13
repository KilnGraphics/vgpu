extern crate hyper;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, RwLock};

use hyper::{Body, Request, Response, Server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

/// Start the server from given config file path
pub async fn start(cope: Arc<RwLock<Option<Box<dyn Fn(String)>>>>) {
    println!("Setting up...");

    let make_svc = make_service_fn(|_: &AddrStream| {
        async move {
            Ok::<_, Infallible>(service_fn(move |request: Request<Body>| async move {
                let payload = hyper::body::to_bytes(request.into_body()).await.unwrap();
                let json = String::from_utf8(payload.to_vec()).unwrap();

                (cope.lock().unwrap().unwrap())(json);
                Ok::<_, Infallible>(Response::new(Body::empty()))
            }))
        }
    });

    // Setup server
    let addr: SocketAddr = "127.0.0.1:9000"
        .parse()
        .expect("Unable to parse host address");
    let ip_type = if addr.is_ipv4() { "IPv4" } else { "IPv6" };
    println!(
        "Listening on {} address {}:{}",
        ip_type,
        &addr.ip(),
        &addr.port()
    );

    let server = Server::bind(&addr)
        .serve(make_svc);

    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
    println!("Started");
}