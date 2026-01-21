use std::{fs::{self, DirEntry}};

use crate::ignores::Ignore;

pub struct PathTree{
    root: String,
    children: Vec<PathTree>,
    files: Vec<DirEntry>,
}

impl PathTree {
    pub fn new(root_path: &str, ignore: &Ignore) -> Self {
        let mut file_tree = PathTree {
            root: root_path.to_string(),
            children: Vec::new(),
            files: Vec::new(),
        };
        for entry in fs::read_dir(root_path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                let mut tree = PathTree {
                    root: entry.file_name().to_string_lossy().to_string(),
                    children: Vec::new(),
                    files: Vec::new(),
                };
                get_path_tree(&entry, &mut tree, &ignore);
                file_tree.children.push(tree);
            } else {
                file_tree.files.push(entry);
            }
        }
        file_tree
    }

    pub fn print_tree(&self) {
        let mut stack = vec![(self, 0)];
        println!("{}", self.root);
        while let Some((node, depth)) = stack.pop() {
            let indent = "│   ".repeat(depth);
            for file in &node.files {
                println!("{}├── {}", indent, file.file_name().to_string_lossy());
            }
            for child in node.children.iter().rev() {
                println!("{}├── {}/", indent, child.root);
                stack.push((child, depth + 1));
            }
        }
    }

    pub fn print_files(&self) {
        let mut out: String = String::new();
        let mut stack = vec![self];
        while let Some(node) = stack.pop() {
            for file in &node.files {
                out.push_str(&format!("========================================\n{}\n========================================\n", file.path().to_string_lossy()));
                let contents = fs::read_to_string(file.path()).expect("Failed to read file");
                out.push_str(&contents);
                out.push_str("\n\n");
            }
            for child in &node.children {
                stack.push(child);
            }
        }
        println!("{}", out);
    }
}

fn get_path_tree(root: &DirEntry, tree: &mut PathTree, ignore: &Ignore) {
    let root = fs::read_dir(root.path()).unwrap();
    // Iterate over the inside directories
    for entry in root {
        let entry = entry.unwrap();
        let mut child_tree = PathTree {
            root: entry.file_name().to_string_lossy().to_string(),
            children: Vec::new(),
            files: Vec::new(),
        };
        if ignore.is_ignored(&entry.path().to_string_lossy()) {
            continue;
        }
        if entry.path().is_dir() {
            get_path_tree(&entry, &mut child_tree, ignore);
            tree.children.push(child_tree);
        } else {
            tree.files.push(entry);
        }
    }
}
