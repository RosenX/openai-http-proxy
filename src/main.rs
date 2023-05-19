use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use hyper_rustls::HttpsConnector;
use std::convert::Infallible;
use std::net::SocketAddr;

type HttpClient = Client<HttpsConnector<HttpConnector>>;

static OPENAI_URL: &str = "https://api.openai.com";

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on {}", addr);

    if let Err(e) = server.await {
        println!("server error: {}", e);
    }
}

async fn proxy(_client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path().to_string();
    let target_url = format!("{}{}", OPENAI_URL, path);

    let mut request_builder = Request::builder().method(req.method()).uri(target_url);

    if req.headers().contains_key("Authorization") {
        request_builder = request_builder.header(
            hyper::header::AUTHORIZATION,
            req.headers().get("Authorization").unwrap().to_owned(),
        );
    } else {
        return Ok(Response::new(Body::from("Missing Authorization header")));
    }

    request_builder = request_builder.header(
        hyper::header::CONTENT_TYPE,
        hyper::header::HeaderValue::from_static("application/json"),
    );

    let new_request = request_builder.body(req.into_body()).unwrap();

    _client.request(new_request).await
}
