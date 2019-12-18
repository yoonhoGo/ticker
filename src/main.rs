extern crate chrono;

use chrono::{TimeZone, Utc};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

fn tick(nano: &mut u32) -> i64 {
    let utc = Utc::now();
    let nanos = utc.timestamp_subsec_nanos() + *nano;
    let dt = Utc.timestamp(utc.timestamp(), nanos);

    *nano += 1;

    dt.timestamp_nanos()
}

async fn get_tick(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    let mut nano = 132;

    let body = tick(&mut nano).to_string();

    *response.body_mut() = Body::from(body);

    Ok(response)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| {
        async {
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(get_tick))
        }
    });

    println!("Server is run");

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
