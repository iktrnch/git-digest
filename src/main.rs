use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    folder: String,
}

struct PathTree{
    root: String,
    children: Vec<PathTree>,
}

impl PathTree {
    fn print(&self, depth: usize) {
        for _ in 0..depth {
            print!("  ");
        }
        println!("{}", self.root);
        for child in &self.children {
            child.print(depth + 1);
        }
    }
}

fn get_path_tree(tree: &mut PathTree) {
    let root = fs::read_dir(&tree.root).unwrap();
    // Iterate over the inside directories
    for entry in root {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let mut child_tree = PathTree {
                root: path.to_str().unwrap().to_string(),
                children: Vec::new(),
            };
            get_path_tree(&mut child_tree);
            // Append recursively found children to the current tree
            tree.children.push(child_tree);
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("Folder provided: {}", args.folder);
    // find all .txt files in the provided folder
    let mut tree = PathTree {
        root: args.folder,
        children: Vec::new(),
    };
    get_path_tree(&mut tree);
    tree.print(0);
}