//! Default Compute@Edge template program.

use fastlazy::RequestLazy;
use fastly::http::{HeaderValue};
use fastly::{Body, Error, Request, RequestExt, ResponseExt};

const BACKEND_NAME: &str = "backend_name";

#[fastly::main]
fn main(mut req: Request<Body>) -> Result<impl ResponseExt, Error> {
    // Make any desired changes to the client request.
    req.headers_mut()
        .insert("Host", HeaderValue::from_static("example.com"));

    req = req
        .set_header("Host", "example.com")?
        .set_header("fastly-debug", "1")?;

    Ok(req.send(BACKEND_NAME)?)
}
