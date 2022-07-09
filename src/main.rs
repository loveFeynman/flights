mod flights;

use futures::{future, Future, Stream};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Error};
use hyper::service::service_fn;
use flights::detect_route;

fn microservice_handler(req: Request<Body>)
    -> Box<dyn Future<Item=Response<Body>, Error=Error> + Send>
{
    match (req.method(), req.uri().path().to_owned().as_ref()) {
        (&Method::POST, "/flights") => {
            let body = req.into_body()
                .concat2()
                .map(|chunk| {
                    chunk.to_vec()
                })
                .map(|resp| {
                    println!("{:?}", String::from_utf8(resp.clone()).unwrap());
                    // Response::new(detect_route(String::from_utf8(resp).unwrap()).unwrap().as_bytes().to_vec().into())
                    Response::new(detect_route(String::from_utf8(resp).unwrap()).unwrap().as_bytes().to_vec().into())

                });

            Box::new(body)
        },
        _ => {
            response_with_code(StatusCode::NOT_FOUND)
        },
    }
}

fn response_with_code(status_code: StatusCode)
    -> Box<dyn Future<Item=Response<Body>, Error=Error> + Send>
{
    let resp = Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap();
    Box::new(future::ok(resp))
}

fn main() {
    actix::run(|| {
        let addr = ([127, 0, 0, 1], 8080).into();
        let builder = Server::bind(&addr);
        let server = builder.serve(move || {
            service_fn(move |req| microservice_handler(req))
        });
        server.map_err(drop)
    });
}