mod messages;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::time::Instant;

fn main() {
    println!("\n{}", messages::WELCOME);

    println!("{}", messages::HELP);

    loop {
        print!("\n{}", messages::PROMPT);

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect(messages::FAILED_TO_READ_INPUT);

        let input = input.trim();

        match input {
            "help" => println!("{}", messages::HELP),
            "connect" => println!("Connecting..."),
            "query" => {
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
            "exit" => break,
            _ => println!("Command not recognized."),
        }
    }
}
