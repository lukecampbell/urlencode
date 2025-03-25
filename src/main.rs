use clap::Parser;
use urlencoding::encode;

#[derive(Parser)]
struct Args {
    /// The thing to encode using RFC 3986 URL Path and Query encoding
    target: Vec<String>,
}

/// Parse and print the command arguments as a RFC-3986 compatible URL Encoded
/// String.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let target = args.target.join(" ");

    println!("{}", encode(&target));

    Ok(())
}
