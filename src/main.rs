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
    folder: String,
}

fn main() {
    let args = Args::parse();
    let ignore = Ignore::new(format!("{}/.gitignore", args.folder).as_str());
    let file_tree = PathTree::new(&args.folder, &ignore);
    file_tree.print_tree();
    file_tree.print_files();
}