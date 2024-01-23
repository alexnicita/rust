use std::{thread, time::Duration};
use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

// fn main() {
//     let pattern = std::env::args().nth(1).expect("no pattern given");
//     let path = std::env::args().nth(2).expect("no path given");

//     let args = Cli {
//         pattern: pattern,
//         path: std::path::PathBuf::from(path),
//     };

//     println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
// }

// #[derive(Debug)]
// struct CustomError(String);

// fn main() -> Result<()> {
//     let args = Cli::parse();

//     println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

//     let content = std::fs::read_to_string(&args.path)
//         .with_context(|| format!("could not read file `{}`", args.path.display()))?;
//     // println!("file content: {}", content);

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }

//     Ok(())

// }

fn main() -> Result<()> {

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    thread::sleep(Duration::from_secs(2));

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

// Run with cargo run -- main src/main.rs

// Guide to publish crates:
// [package]
// name = "grrs"
// version = "0.1.0"
// authors = ["Your Name <your@email.com>"]
// license = "MIT OR Apache-2.0"
// description = "A tool to search files"
// readme = "README.md"
// homepage = "https://github.com/you/grrs"
// repository = "https://github.com/you/grrs"
// keywords = ["cli", "search", "demo"]
// categories = ["command-line-utilities"]
