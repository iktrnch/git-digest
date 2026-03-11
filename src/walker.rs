use crate::Ignore;
use ignore::DirEntry;

use ignore::WalkBuilder;

struct FileTree {
    /// Path to the directory
    root: String,
    /// Vector of paths of child directories
    children: Vec<FileTree>,
    /// Vector of file paths in current directory
    files: Vec<DirEntry>,
}

impl FileTree {
    pub fn new(root_path: &str) -> Self {
        FileTree {
            root: root_path.to_string(),
            children: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Appends a child directory to the data structure
    pub fn append_child(&mut self, child: FileTree) {
        self.children.push(child);
    }

    /// Appends files in the directory to the datastructure
    pub fn append_file(&mut self, file: DirEntry) {
        self.files.push(file);
    }

    pub fn walk_dirs(&mut self) {
        // Get the children of the dir
        let children = WalkBuilder::new(&self.root)
            .min_depth(None)
            .max_depth(Some(1))
            .build();

        // Append the entries to the tree
        for entry in children {
            let entry = entry.unwrap();

            if entry.path().is_dir() {
                // Skip the root itself (the "." entry)
                let path = entry.path().to_str().unwrap();
                if path == self.root {
                    continue;
                }
                self.append_child(FileTree::new(path));
            } else if entry.path().is_file() {
                self.append_file(entry);
            }
        }

        for child in &mut self.children {
            child.walk_dirs();
        }
    }

    fn print_children(&self, depth: usize, out: &mut String) {
        for (i, child) in self.children.iter().enumerate() {
            let dir_name = match self.root.rsplit_once("/") {
                Some(name) => name.1,
                None => self.root.as_str(),
            };

            let prefix = if self.children.len() == i + 1 && self.files.len() != 0 {
                "└─"
            } else {
                "├─"
            };

            let entry = format!(
                "{}{} {}\n{}",
                "│ ".repeat(depth),
                prefix,
                dir_name,
                child.print(depth + 1)
            );
            out.push_str(&entry);
        }
    }

    fn print_files(&self, depth: usize, out: &mut String) {
        for (i, file) in self.files.iter().enumerate() {
            let prefix = if self.files.len() == i + 1 {
                "└─"
            } else {
                "├─"
            };

            let entry = format!(
                "{}{} {}\n",
                "│ ".repeat(depth),
                prefix,
                file.file_name().to_str().unwrap()
            );
            out.push_str(&entry);
        }
    }

    pub fn print(&self, depth: usize) -> String {
        let mut out: String = String::new();

        self.print_children(depth, &mut out);
        self.print_files(depth, &mut out);

        out
    }
}

/// Wrapper struct for file walker
pub struct Walker {
    /// Stores creates an in-memory representation of the directory
    file_tree: FileTree,
    /// Stores the ignore patterns for each directory, where the key is the directory name and the value is a vector of ignore patterns for that directory
    /// This allows to use ignore patterns from parent directories without copying them
    ignore: Ignore,
}

impl Walker {
    pub fn new(root_path: &str) -> Self {
        Walker {
            file_tree: FileTree::new(root_path),
            ignore: Ignore::new(),
        }
    }

    /// Recursively walks through every directory and file starting from the root path
    /// And applies ignore patterns and building the file tree structure.
    /// The directory tree is traversed using BFS
    pub fn walk_dirs(&mut self) {
        self.file_tree.walk_dirs();
    }

    pub fn print_tree(&self) {
        println!("{}", self.file_tree.print(0));
    }
}
