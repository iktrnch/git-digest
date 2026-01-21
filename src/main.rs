mod pathtree;
mod ignores;

use clap::Parser;
use pathtree::PathTree;
use ignores::Ignore;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = ".")]
    folder: String,
}

fn main() {
    let args = Args::parse();
    let ignore = Ignore::new(".gitignore");
    let file_tree = PathTree::new(&args.folder, &ignore);
    file_tree.print();
}