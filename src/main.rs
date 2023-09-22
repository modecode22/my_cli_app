use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // Optional name provided
    if let Some(name) = &cli.name {
        println!("Doing something with name: {}", name);
    }

    // Custom config file
    if let Some(config_path) = &cli.config {
        println!("Reading config from: {}", config_path.display());
        // Read the config file and do something...
    }

    // Debug level
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Debug mode is at an extreme level"),
    }

    // Subcommands
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
                // Actually print the list or do something else...
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {
            println!("No subcommand provided.");
        }
    }

    // Now that all the flags and options are parsed, here you can insert whatever "cool stuff" your app does.
    if cli.debug > 0 {
        println!("Doing cool stuff with debug information...");
    }

    if let Some(_) = &cli.name {
        // Do cool stuff with the name...
    }
}
