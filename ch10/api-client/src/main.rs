use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use colored_json::ToColoredJson;
use http_body_util::BodyExt;
use hyper::{Method, Request, Uri, body::Bytes, header::CONTENT_TYPE};
use hyper_util::rt::TokioIo;
use serde_json::json;
use tokio::net::TcpStream;
use yansi::Paint;

#[derive(Debug)]
struct Cli {
    uri: hyper::Uri,
    command: Commands,
}

#[derive(Debug)]
enum Commands {
    List,
    Create {
        body: String,
    },
    Read {
        id: i64,
    },
    Update {
        id: i64,
        body: String,
        completed: bool,
    },
    Delete {
        id: i64,
    },
}

fn get_matches() -> ArgMatches {
    Command::new("api-client")
        .version("0.0.1")
        .arg(
            Arg::new("uri")
                .value_name("URL")
                .required(true)
                .value_parser(value_parser!(hyper::Uri))
                .help("Base URL of API service"),
        )
        .subcommand(Command::new("list").about("List all todos"))
        .subcommand(
            Command::new("create").about("Create a new todo").arg(
                Arg::new("body")
                    .value_name("BODY")
                    .help("The todo body")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("read").about("Read a todo").arg(
                Arg::new("id")
                    .value_name("ID")
                    .required(true)
                    .help("The todo ID")
                    .value_parser(value_parser!(i64)),
            ),
        )
        .subcommand(
            Command::new("update")
                .about("Update a todo")
                .arg(
                    Arg::new("id")
                        .value_name("ID")
                        .required(true)
                        .help("The todo ID")
                        .value_parser(value_parser!(i64)),
                )
                .arg(
                    Arg::new("body")
                        .value_name("BODY")
                        .help("The todo body")
                        .required(true),
                )
                .arg(
                    Arg::new("completed")
                        .help("Mark todo as completed")
                        .action(ArgAction::SetTrue)
                        .short('c')
                        .long("completed"),
                ),
        )
        .subcommand(
            Command::new("delete").about("Delete a todo").arg(
                Arg::new("id")
                    .value_name("ID")
                    .required(true)
                    .help("The todo ID")
                    .value_parser(value_parser!(i64)),
            ),
        )
        .subcommand_required(true)
        .get_matches()
}

fn parse_cli(arg_matches: ArgMatches) -> Cli {
    let uri = arg_matches.get_one::<hyper::Uri>("uri").cloned().unwrap();
    let command = match arg_matches.subcommand() {
        Some(("list", _)) => Commands::List,
        Some(("create", matches)) => {
            let body = matches.get_one::<String>("body").cloned().unwrap();
            Commands::Create { body }
        }
        Some(("read", matches)) => {
            let id = matches.get_one::<i64>("id").cloned().unwrap();
            Commands::Read { id }
        }
        Some(("update", matches)) => {
            let id = matches.get_one::<i64>("id").cloned().unwrap();
            let body = matches.get_one::<String>("body").cloned().unwrap();
            let completed = matches.get_flag("completed");
            Commands::Update {
                id,
                body,
                completed,
            }
        }
        Some(("delete", matches)) => {
            let id = matches.get_one::<i64>("id").cloned().unwrap();
            Commands::Delete { id }
        }
        _ => unreachable!(),
    };

    Cli { uri, command }
}

async fn request(
    uri: hyper::Uri,
    method: Method,
    body: Option<String>,
) -> Result<(), anyhow::Error> {
    let host = uri.host().expect("uri ha no host");
    let port = uri.port_u16().unwrap_or(80);
    let stream = TcpStream::connect((host, port)).await?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection failed: {:?}", e);
        }
    });

    let mut req = Request::builder().uri(&uri).method(method);
    if let Some(authority) = uri.authority() {
        req = req.header(hyper::header::HOST, authority.as_str());
    }
    let req = req
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(
            body.map(|s| http_body_util::Full::new(Bytes::copy_from_slice(s.as_bytes())))
                .unwrap_or_default(),
        )?;

    let mut res = sender.send_request(req).await?;

    let mut buf = Vec::new();
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            buf.extend_from_slice(chunk);
        }
    }

    let s = String::from_utf8(buf)?;

    eprintln!("Status: {}", Paint::green(&res.status().as_u16()));
    if res.headers().contains_key(CONTENT_TYPE) {
        let content_type = res.headers().get(CONTENT_TYPE).unwrap().to_str()?;
        eprintln!("Content-Type: {}", Paint::green(content_type));
        if content_type.starts_with("application/json") {
            println!("{}", s.to_colored_json_auto()?);
        } else {
            println!("{}", s);
        }
    } else {
        println!("{}", s);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = get_matches();
    let cli = parse_cli(matches);

    let mut uri_builder = Uri::builder();
    if let Some(scheme) = cli.uri.scheme() {
        uri_builder = uri_builder.scheme(scheme.clone());
    }

    if let Some(authority) = cli.uri.authority() {
        uri_builder = uri_builder.authority(authority.clone());
    }

    match cli.command {
        Commands::List => {
            request(
                uri_builder.path_and_query("/v1/todos").build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Delete { id } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::DELETE,
                None,
            )
            .await
        }
        Commands::Read { id } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Create { body } => {
            request(
                uri_builder.path_and_query("/v1/todos").build()?,
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
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::PUT,
                Some(json!({"body": body, "completed": completed}).to_string()),
            )
            .await
        }
    }
}
