use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use hyper_tls::HttpsConnector;
use std::convert::Infallible;
use std::net::SocketAddr;

type HttpClient = Client<HttpsConnector<HttpConnector>>;

static OPENAI_URL: &str = "https://api.openai.com";

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);

    println!("Listening on {}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(
    _client: HttpClient,
    mut req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path().to_string();
    let target_url = format!("{}{}", OPENAI_URL, path);

    *req.uri_mut() = target_url.parse().unwrap();

    _client.request(req).await.map_err(|err| {
        println!("Error: {}", err);
        err
    })
}
