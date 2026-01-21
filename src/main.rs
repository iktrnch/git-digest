mod pathtree;
mod ignores;

use clap::Parser;
use pathtree::PathTree;
use ignores::Ignore;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to digest
    #[arg(short, long, default_value = ".")]
    directory: String,

    /// Path to ignore file
    #[arg(short, long, default_value = "./.gitignore")]
    ignore: String,

    /// Output file tree structure
    #[arg(short, long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    tree: bool,

    /// Output file contents
    #[arg(short, long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    files: bool,
}

fn main() {
    let args = Args::parse();
    let ignore = Ignore::new(&args.ignore);
    let file_tree = PathTree::new(&args.directory, &ignore);
    if args.tree {
        file_tree.print_tree();
    }
    if args.files {
        file_tree.print_files();
    }
}