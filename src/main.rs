use clap::Parser;
use std::{fs::{self, DirEntry}};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = ".")]
    folder: String,
}

struct PathTree{
    children: Vec<PathTree>,
    files: Vec<DirEntry>,
}

impl PathTree {
    fn print(&self) {
        let mut stack = vec![(self, 0)];
        while let Some((node, depth)) = stack.pop() {
            let indent = "│   ".repeat(depth);
            for file in &node.files {
                println!("{}├── {}", indent, file.file_name().to_string_lossy());
            }
            for child in node.children.iter().rev() {
                stack.push((child, depth + 1));
            }
        }
    }
}

fn get_path_tree(root: &DirEntry, tree: &mut PathTree) {
    let root = fs::read_dir(root.path()).unwrap();
    // Iterate over the inside directories
    for entry in root {
        let entry = entry.unwrap();
        let mut child_tree = PathTree {
            children: Vec::new(),
            files: Vec::new(),
        };
        if entry.path().is_dir() {
            get_path_tree(&entry, &mut child_tree);
            tree.children.push(child_tree);
        } else {
            tree.files.push(entry);
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("Folder provided: {}", args.folder);
    // find all directories in the provided folder
    let mut file_tree = PathTree {
        children: Vec::new(),
        files: Vec::new(),
    };
    for entry in fs::read_dir(&args.folder).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            let mut tree = PathTree {
                children: Vec::new(),
                files: Vec::new(),
            };
            get_path_tree(&entry, &mut tree);
            file_tree.children.push(tree);
        }
    }
    file_tree.print();
}