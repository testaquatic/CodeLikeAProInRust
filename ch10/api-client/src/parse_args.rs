//! 되도록 매크로를 사용하지 않고 작성해봤다.
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

pub struct Cli {
    url: hyper::Uri,
    command: Commands,
}

impl Cli {
    pub fn get_url(&self) -> hyper::Uri {
        self.url.clone()
    }

    pub fn get_command(self) -> Commands {
        self.command
    }
}

pub enum Commands {
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

pub fn get_cli_args() -> Cli {
    let matches = get_matches();
    get_cli(matches)
}

pub fn get_matches() -> ArgMatches {
    Command::new("api-client")
        .subcommand_required(true)
        .subcommand(
            Command::new("list")
                .args_conflicts_with_subcommands(true)
                .about("List all todos"),
        )
        .subcommand(
            Command::new("create")
                .args_conflicts_with_subcommands(true)
                .about("Create a new todo")
                .arg(
                    Arg::new("body")
                        .value_name("BODY")
                        .required(true)
                        .help("The todo body")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("read")
                .args_conflicts_with_subcommands(true)
                .about("Read a todo")
                .arg(
                    Arg::new("id")
                        .value_name("ID")
                        .required(true)
                        .help("The todo ID")
                        .value_parser(value_parser!(i64)),
                ),
        )
        .subcommand(
            Command::new("update")
                .args_conflicts_with_subcommands(true)
                .about("Update a todo")
                .arg(
                    Arg::new("id")
                        .value_name("ID")
                        .required(true)
                        .num_args(1)
                        .value_parser(value_parser!(i64))
                        .help("The todo ID"),
                )
                .arg(
                    Arg::new("body")
                        .value_name("BODY")
                        .required(true)
                        .num_args(1)
                        .help("The todo body"),
                )
                .arg(
                    Arg::new("completed")
                        .long("completed")
                        .short('c')
                        .help("Mark todo as completed")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("delete")
                .args_conflicts_with_subcommands(true)
                .about("Delete a todo")
                .arg(
                    Arg::new("id")
                        .value_name("ID")
                        .num_args(1)
                        .required(true)
                        .value_parser(value_parser!(i64))
                        .help("The todo ID"),
                ),
        )
        .arg(
            Arg::new("url")
                .value_name("URL")
                .num_args(1)
                .value_parser(value_parser!(hyper::Uri))
                .help("Base URL of API service")
                .required(true),
        )
        .get_matches()
}

pub fn get_cli(matches: ArgMatches) -> Cli {
    let url = matches
        .get_one::<hyper::Uri>("url")
        .cloned()
        .expect("The `URL` must be present.");

    match matches.subcommand() {
        Some(("list", _)) => Cli {
            url,
            command: Commands::List,
        },
        Some(("create", create_matches)) => {
            let body = create_matches
                .get_one::<String>("body")
                .cloned()
                .expect("The `BODY` must be present.");

            Cli {
                url,
                command: Commands::Create { body },
            }
        }
        Some(("read", read_matches)) => {
            let id = read_matches
                .get_one::<i64>("id")
                .cloned()
                .expect("The `ID` must be present.");

            Cli {
                url,
                command: Commands::Read { id },
            }
        }
        Some(("update", update_matches)) => {
            let id = update_matches
                .get_one::<i64>("id")
                .cloned()
                .expect("The `ID` must be present.");
            let body = update_matches
                .get_one::<String>("body")
                .cloned()
                .expect("The `BODY` must be present.");
            let completed = update_matches.get_flag("completed");

            Cli {
                url,
                command: Commands::Update {
                    id,
                    body,
                    completed,
                },
            }
        }
        Some(("delete", delete_matches)) => {
            let id = delete_matches
                .get_one::<i64>("id")
                .cloned()
                .expect("The `ID` must be present.");

            Cli {
                url,
                command: Commands::Delete { id },
            }
        }
        _ => unreachable!(),
    }
}
