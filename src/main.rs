// Modules
mod server;
mod filesystem;
mod config;

// Crates
use kagero::printer::{Colors, Printer};
use std::env;
use std::process;
use std::path::Path;
use crate::config::{Config};

// Main
fn main() {
    let mut printer = Printer::default();

    // Start Actix-Web Server with development config
    #[cfg(debug_assertions)]
    let server_result = server::server(
        get_config(
            env::current_dir().unwrap()
            .join("dev-config.json")
        )
    );

    // Start Actix-Web Server with production config
    #[cfg(not(debug_assertions))]
    let server_result = server::server(get_config("/etc/raspberrypi/config.json"));

    match server_result {
        Ok(_) => {
            printer.println("Shuting down the server...", Colors::WhiteBright);
            process::exit(0)
        },
        Err(err) => {
            printer.println("An error occured!", Colors::RedBright);
            printer.errorln(err.to_string().as_str(), Colors::Red)
        }
    }
}

fn get_config(path: impl AsRef<Path>) -> Config {
    match config::read_config(path) {
        Ok(conf) => conf,
        Err(errmsg) => {
            Printer::default().errorln(errmsg, Colors::Red);
            process::exit(2);            
        } 
    }
}