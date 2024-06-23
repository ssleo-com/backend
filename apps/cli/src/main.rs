mod messages;

use clap::Parser;
use messages::{EntityType, RustflixArgs};

fn main() {
    println!("Welcome to Rustflix interactive session.");
    println!("Type 'exit' to quit.");

    // Manually handle argument parsing to prevent automatic program exit
    let args_result = RustflixArgs::parse_from(std::env::args_os());

    // Check if parsing was successful or handle errors
    match args_result {
        args => {
            match args.entity_type {
                EntityType::User(user) => println!("User: {:?}", user),
                EntityType::Video(video) => println!("Video: {:?}", video),
            };
        }
    }
}
