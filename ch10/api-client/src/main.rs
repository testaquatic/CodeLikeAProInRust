use api_client::parse_args::{get_cli_args, Commands};
use colored_json::ToColoredJson;
use http::header::CONTENT_TYPE;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty};
use hyper::{
    client::{self},
    Method, Request,
};
use hyper_util::rt::TokioIo;
use serde_json::json;
use tokio::net::TcpStream;
use yansi::Paint;

async fn request(
    url: hyper::Uri,
    method: Method,
    body: Option<String>,
) -> Result<(), anyhow::Error> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(e) = conn.await {
            println!("Connection failed {:?}", e);
        }
    });

    let req = Request::builder()
        .uri(url)
        .method(method)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(body.map(BoxBody::new).unwrap_or(BoxBody::new(Empty::new())))?;
    let mut res = sender.send_request(req).await?;

    let mut buf = Vec::new();
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            buf.extend_from_slice(chunk);
        }
    }

    let s = String::from_utf8(buf)?;

    eprintln!("Status: {}", res.status().green());
    if let Some(content_type) = res.headers().get(CONTENT_TYPE) {
        let content_type = content_type.to_str()?;
        eprint!("Content-Type: {}", content_type.green());
        if content_type.starts_with("application/json") {
            println!("{}", s.to_colored_json_auto()?);
        } else {
            println!("{}", &s);
        }
    } else {
        println!("{}", &s);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = get_cli_args();

    let cli_url = cli.get_url();
    let mut url_builder = hyper::Uri::builder();
    if let Some(scheme) = cli_url.scheme() {
        url_builder = url_builder.scheme(scheme.clone());
    }

    if let Some(authority) = cli_url.authority() {
        url_builder = url_builder.authority(authority.clone());
    }

    match cli.get_command() {
        Commands::List => {
            request(
                url_builder.path_and_query("/v1/todos").build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Delete { id } => {
            request(
                url_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::DELETE,
                None,
            )
            .await
        }
        Commands::Read { id } => {
            request(
                url_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Create { body } => {
            request(
                url_builder.path_and_query("/v1/todos").build()?,
                Method::POST,
                Some(json!({"body": body}).to_string()),
            )
            .await
        }
        Commands::Update {
            id,
            body,
            completed,
        } => {
            request(
                url_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::PUT,
                Some(json!({"body": body, "completed": completed}).to_string()),
            )
            .await
        }
    }?;

    Ok(())
}
