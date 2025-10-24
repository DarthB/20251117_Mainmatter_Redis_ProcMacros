# Gem No. 2 - Clap

> By Kevin B. Knapp and many Contributors.

- [`Clap`](https://docs.rs/clap/latest/clap/) - Expressive Command Line Parsing without Boilerplate.

```rust
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
```