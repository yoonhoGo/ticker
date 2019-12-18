extern crate chrono;

use chrono::{TimeZone, Utc};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(get_tick)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
