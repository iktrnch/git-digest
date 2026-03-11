mod digest;

use clap::Parser;
use digest::Digest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to digest
    #[arg(short, long, default_value = ".")]
    directory: String,

    /// Filename of ignore file
    #[arg(short, long, default_value = ".gitignore")]
    ignore: String,

    /// Output file tree structure
    #[arg(short, long, default_value_t = true, action = clap::ArgAction::SetTrue)]
    tree: bool,

    /// Output file contents
    #[arg(short, long, default_value_t = true, action = clap::ArgAction::SetTrue)]
    files: bool,

    /// Only output files that match the regex pattern
    /// Example: --pattern ".*\\.rs$" to only include Rust files
    /// Note: This will be applied after the ignore rules, so it will only filter the files that are not ignored
    #[arg(short, long, default_value = "")]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    let digest = Digest::new(&args.directory, &args.pattern);

    if args.tree {
        digest.print_tree();
    }
    // if args.files {
    //     digest.print_files();
    // }
}
