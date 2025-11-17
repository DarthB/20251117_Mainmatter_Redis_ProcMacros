# Readme

## Slides

Either [download the PDF](20251117_Mainmatter_Redis_ProcMacros.pdf)

> or Alternatively use mdbook directly:

To see the slides run `mdbook serve` in the root folder of that repository. 

You can install `mdbook` with

```
cargo install cargo-mdbook
```

or with a a package manager.

## Code

The code is the `code` subdirectory just change in one of the directories, e.g. `code/c_mypoint_clap`, and use:

- `cargo run` to run the example.
- `cargo expand > tmp.rs` to write the fully expanded macro code in the file `tmp.rs`.

You can install `cargo expand` with:

```
cargo install cargo-expand
```
