use ignore::WalkBuilder;
use regex::Regex;

#[derive(Debug)]
struct FileTree {
    /// Path to the directory
    root: String,
    /// Vector of paths of child directories
    children: Vec<FileTree>,
    /// Vector of file paths in current directory
    files: Vec<String>,
}

impl FileTree {
    pub fn new(root_path: &str) -> Self {
        FileTree {
            root: root_path.to_string(),
            children: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Inserts an entry into the tree
    /// Recursively inserts directories before the file
    pub fn insert(&mut self, path: &str) {
        let path = match path.split_once("/") {
            None => {
                // We reached the file - insert
                self.append_file(path);
                return;
            }
            Some(p) => p,
        };
        // Check if file is in root dir
        if self.root == path.0 {
            self.insert(path.1);
            return;
        }

        // Find the dir to insert to
        for child in &mut self.children {
            if child.root == path.0 {
                child.insert(path.1);
                return;
            }
        }
        // If the directory doesnt exist - create it
        let mut new_child = FileTree::new(path.0);
        new_child.insert(path.1);
        self.append_child(new_child);
    }

    /// Appends a child directory to the data structure
    fn append_child(&mut self, child: FileTree) {
        self.children.push(child);
    }

    /// Appends files in the directory to the datastructure
    fn append_file(&mut self, file: &str) {
        self.files.push(file.to_string());
    }

    fn print_children(&self, depth: &str, out: &mut String) {
        for (i, child) in self.children.iter().enumerate() {
            // Create appropriate indentation
            let mut child_depth = "│   ";
            let mut prefix = "├── ";

            if self.children.len() == i + 1 {
                prefix = "└── ";
                child_depth = "   ";
            }

            let child_depth = format!("{}{} ", depth, child_depth);

            out.push_str(&format!("{}{}{}", depth, prefix, child.print(&child_depth)));
        }
    }

    fn print_files(&self, depth: &str, out: &mut String) {
        for (i, file) in self.files.iter().enumerate() {
            let prefix = if self.files.len() == i + 1 && self.children.len() == 0 {
                "└── "
            } else {
                "├── "
            };

            out.push_str(&format!("{}{}{}\n", depth, prefix, file));
        }
    }

    pub fn print(&self, depth: &str) -> String {
        let mut out: String = String::new();

        out.push_str(&format!("{}\n", self.root));
        self.print_files(depth, &mut out);
        self.print_children(depth, &mut out);

        out
    }
}

/// Wrapper struct for file walker
pub struct Digest {
    /// Stores creates an in-memory representation of the directory
    file_tree: FileTree,
}

impl Digest {
    pub fn new(root_path: &str, re_str: &str) -> Self {
        Digest {
            file_tree: Self::walk_dirs(root_path, re_str),
        }
    }

    /// Recursively walks through every directory and file starting from the root path
    /// And applies ignore patterns and building the file tree structure.
    /// The directory tree is traversed using BFS
    fn walk_dirs(path: &str, re_str: &str) -> FileTree {
        let root = match path.split_once("/") {
            None => path,
            Some(val) => val.0,
        };
        let mut tree = FileTree::new(root);
        let re = Regex::new(re_str).unwrap();

        let entries = WalkBuilder::new(path).build();
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path().to_str().unwrap();
                    if entry.path().is_file() && re.is_match(path) {
                        tree.insert(path);
                    }
                }
                Err(e) => {
                    eprintln!("ERROR: Could not access file\n{}", e);
                }
            }
        }
        tree
    }

    pub fn print_tree(&self) {
        println!("{}", self.file_tree.print(""));
    }
}
