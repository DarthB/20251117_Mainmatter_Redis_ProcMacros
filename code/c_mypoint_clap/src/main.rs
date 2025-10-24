use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct MyPoint {
    x: f64,
    y: f64,
}

impl MyPoint {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Display for MyPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyPoint(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Parser)]
#[command(name = "mypoint")]
#[command(about = "A CLI for managing MyPoint objects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new point with x and y coordinates
    Add {
        /// X coordinate
        x: f64,
        /// Y coordinate
        y: f64,
    },
    /// List all points
    List,
    /// Quit the application
    Quit,
}

fn main() {
    let mut points: Vec<MyPoint> = Vec::new();

    println!("MyPoint CLI - Interactive Mode");
    println!("Available commands: add <x> <y>, list, quit");
    println!("Type 'help' for more information\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                // Parse the input as command line arguments
                let args: Vec<&str> = input.split_whitespace().collect();
                if args.is_empty() {
                    continue;
                }

                // Prepend program name for clap parsing
                let mut full_args = vec!["mypoint"];
                full_args.extend(args);

                match Cli::try_parse_from(full_args) {
                    Ok(cli) => match cli.command {
                        Commands::Add { x, y } => {
                            let point = MyPoint::new(x, y);
                            points.push(point);
                            println!("Added: {}", point);
                        }
                        Commands::List => {
                            if points.is_empty() {
                                println!("No points stored.");
                            } else {
                                println!("Stored points:");
                                for (i, point) in points.iter().enumerate() {
                                    println!("  {}: {}", i + 1, point);
                                }
                            }
                        }
                        Commands::Quit => {
                            println!("Goodbye!");
                            break;
                        }
                    },
                    Err(err) => {
                        println!("Error: {}", err);
                        println!("Type '--help' or '<cmd> --help` for usage information.");
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
    }
}
