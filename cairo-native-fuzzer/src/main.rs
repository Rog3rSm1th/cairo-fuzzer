mod custom_rand;
mod fuzzer;
mod mutator;
mod runner;
mod utils;

use clap::Parser;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::fuzzer::fuzzer::Fuzzer;

/// Command-line arguments for the fuzzer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the Cairo program
    #[arg(short, long)]
    program_path: PathBuf,

    /// Entry point of the Sierra program
    #[arg(short, long)]
    entry_point: Option<String>,

    /// Analyze the program and print function prototypes
    #[arg(short, long, requires = "program_path")]
    analyze: bool,

    /// Number of iterations to use for fuzzing
    #[arg(short, long, default_value_t = -1)]
    iter: i32,
}

fn main() {
    let args = Args::parse();

    // Get the current time as a Unix timestamp
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seed = since_the_epoch.as_secs();

    let mut fuzzer = Fuzzer::new(args.program_path, args.entry_point);

    match fuzzer.init(seed) {
        Ok(()) => {
            // Print the contract functions
            if args.analyze {
                fuzzer.print_functions_prototypes();
            }
            // Run the fuzzer
            else {
                match fuzzer.fuzz(args.iter) {
                    Ok(()) => println!("Fuzzing completed successfully."),
                    Err(e) => eprintln!("Error during fuzzing: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error during initialization: {}", e),
    }
}
