mod file_tree;
mod matcher;

use ignore::WalkBuilder;

use crate::Args;

use file_tree::FileTree;
use matcher::Matcher;

/// Wrapper struct for file walker
pub struct Digest {
    /// Stores creates an in-memory representation of the directory
    file_tree: FileTree,
    matcher: Matcher,
}

impl Digest {
    pub fn new(args: &Args) -> Self {
        // Get the directory for the root of the tree
        let initial_directory = match args.directory.split_once("/") {
            Some(path) => path.0,
            None => args.directory.as_str(),
        };

        Digest {
            file_tree: FileTree::new(initial_directory),
            matcher: Matcher::new(&args.include, &args.exclude),
        }
    }

    /// Recursively walks through every directory and file starting from the root path
    /// And applies ignore patterns and building the file tree structure.
    /// The directory tree is traversed using BFS
    pub fn walk_dirs(&mut self, path: &str) {
        let entries = WalkBuilder::new(path).build();
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path().to_str().unwrap();
                    if entry.path().is_file() && self.matcher.is_match(path) {
                        self.file_tree.insert(path);
                    }
                }
                Err(e) => {
                    eprintln!("ERROR: Could not access file\n{}", e);
                }
            }
        }
    }

    pub fn print_tree(&self) {
        println!("{}", self.file_tree.print(""));
    }
}
