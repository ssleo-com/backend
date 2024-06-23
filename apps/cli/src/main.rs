mod commands;
mod person;
use clap::Parser;
use commands::{EntityType, RustflixArgs};
use person::query;

fn handle_person_command(person_command: commands::PersonCommand) {
    match person_command.command {
        commands::PersonSubcommand::Create(create_person) => {
            println!("Creating person: {create_person:?}");
        }
        commands::PersonSubcommand::Update(update_person) => {
            println!("Updating person: {update_person:?}");
        }
        commands::PersonSubcommand::Delete(delete_entity) => {
            println!("Deleting person: {delete_entity:?}");
        }
        commands::PersonSubcommand::Show => {
            query();
        }
    }
}
fn main() {
    // Manually handle argument parsing to prevent automatic program exit
    let args_result = RustflixArgs::parse_from(std::env::args_os());

    // Check if parsing was successful or handle errors
    match args_result {
        args => {
            match args.entity_type {
                EntityType::Person(person) => handle_person_command(person),
                EntityType::Video(video) => println!("Video: {video:?}"),
            };
        }
    }
}
