extern crate hyper;

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tokio::sync::mpsc::Sender;

/// Start the server from given config file path
pub async fn start(sender: Sender<String>) {
    let service = service_fn(move |request: Request<Body>| {
        let sender = sender.clone();

        async move {
            if let Ok(payload) = hyper::body::to_bytes(request.into_body()).await {
                if let Ok(json) = String::from_utf8(payload.to_vec()) {
                    // We don't really care if the backing receiver did not handle a message
                    #[allow(unused_must_use)]
                    {
                        sender.send(json).await;
                    }
                }
            }

            Ok::<_, Infallible>(Response::new(Body::empty()))
        }
    });

    let make_svc = make_service_fn(|_| {
        let service = service.clone();

        async move { Ok::<_, Infallible>(service) }
    });

    // Setup server
    let addr: SocketAddr = "127.0.0.1:9000"
        .parse()
        .expect("Unable to parse host address");

    println!("Listening on address {}:{}", addr.ip(), addr.port());

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
