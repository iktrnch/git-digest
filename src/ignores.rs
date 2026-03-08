use std::collections::HashMap;
use std::fs;

pub struct Ignore {
    patterns: HashMap<String, Vec<String>>,
}

impl Ignore {
    pub fn new() -> Self {
        Ignore {
            patterns: HashMap::new(),
        }
    }

    /// Fetches and stores regex patterns to be ignored when parsing directories
    pub fn add_pattern_from_file(&mut self, file: &str) {
        //Split by /
        let dir = file
            .rsplit_once("/")
            .map(|(before, _)| before)
            .expect("Failed to parse directory");
        let dir = String::from(dir); // REPLACE LATER (deep copy not efficient)

        // Read .gitignore contents
        let content = fs::read_to_string(file).expect("Failed to read .gitignore");
        let vector = self.patterns.entry(dir).or_insert_with(Vec::new);
        for entry in content.lines() {
            vector.push(String::from(entry));
        }
    }

    /// Checks if the given path matches the regex of any ignore entries
    pub fn has(&self, path: &str) -> bool {
        let path_parts: Vec<&str> = path.split("/").collect();
        for i in path_parts.len() - 1..0 {
            let dir_path = path_parts[..i].join("/");
        }
        true
    }
}
