mod cursecov;

use clap::Parser;
use cursecov::run_cursecov;

/// A tool for analyzing the percentage of curse word comments in JS projects.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pattern for files to include.
    #[arg(long, default_value_t = String::from("**/*.js,**/*.ts"))]
    include_pattern: String,

    /// Pattern for files to ignore.
    #[arg(long, default_value_t = String::from(""))]
    ignore_pattern: String,

    /// Minimum percentage of comments that need curse words.
    #[arg(long, default_value_t = 30.0)]
    min_coverage: f64,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    if args.min_coverage < 0.0 || args.min_coverage > 100.0 {
        return Err(String::from("Min coverage must be between 0 and 100."));
    }

    run_cursecov(args.include_pattern, args.ignore_pattern, args.min_coverage)?;

    Ok(())
}
