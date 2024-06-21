use std::env;

pub fn get_env_var(name: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => {
            println!(
                "{} environment variable is missing. Please add it to your .env file.",
                name
            );
            std::process::exit(1);
        }
    }
}
