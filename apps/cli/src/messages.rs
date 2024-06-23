use std::{process::Command, time::Instant};

// Prompt messages
pub const WELCOME: &str = "Welcome to the CLI!";
pub const HELP: &str = "Type \"help\" for help.";
pub const PROMPT: &str = "ssleo=> ";

// Error messages
pub const FAILED_TO_READ_INPUT: &str = "Failed to read input";

#[derive(PartialEq)]
pub struct CliCommand<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub action: fn(),
    pub args: Vec<&'a str>,
}

impl<'a> CliCommand<'a> {
    pub fn help() {
        println!("");
        for command in COMMANDS.iter().filter(|command| command.name != "help") {
            println!("{} - {}", command.name, command.description);
        }
    }

    pub fn query() {
        let start_time = Instant::now();
        let output = Command::new("curl")
            .arg("localhost:9090/persons/1")
            .output()
            .expect("Failed to execute command");
        let elapsed_time = start_time.elapsed();
        println!("Req duration: {:?}", elapsed_time);
        if output.status.success() {
            let response = String::from_utf8_lossy(&output.stdout);
            println!("Response:\n{}", response);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error executing command:\n{}", error);
        }
    }
}

pub const COMMANDS: [CliCommand; 5] = [
    CliCommand {
        name: "help",
        description: "Prints this help message",
        action: CliCommand::help,
        args: vec![],
    },
    CliCommand {
        name: "connect",
        description: "Connect to the database",
        action: || println!("Connecting..."),
        args: vec![],
    },
    CliCommand {
        name: "query",
        description: "Query the database",
        action: CliCommand::query,
        args: vec![],
    },
    CliCommand {
        name: "exit",
        description: "Exit the CLI",
        action: || std::process::exit(0),
        args: vec![],
    },
    CliCommand {
        name: "clear",
        description: "Clear the screen",
        action: || {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        },
        args: vec![],
    },
];

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show users
    User(UserCommand),

    /// Create, update, delete or show videos
    Video(VideoCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),

    /// Update an existing user
    Update(UpdateUser),

    /// Delete a user
    Delete(DeleteEntity),

    /// Show all users
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The id of the user to update
    pub id: i32,

    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create(CreateVideo),

    /// Update an existing video
    Update(UpdateVideo),

    /// Delete a video
    Delete(DeleteEntity),

    /// Show all videos
    Show,
}

#[derive(Debug, Args)]
pub struct CreateVideo {
    /// The title of the video to create
    pub title: String,

    /// The description of the video to create
    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateVideo {
    /// The id of the video to update
    pub id: i32,

    /// The title of the video
    pub title: String,

    /// The description of the video
    pub description: String,
}
