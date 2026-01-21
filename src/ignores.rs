use std::fs;

pub struct Ignore{
    patterns: Vec<String>,
}

impl Ignore {
    pub fn new(gitignore_path: &str) -> Self {
        let contents = fs::read_to_string(gitignore_path).expect("Failed to read .gitignore file");
        let mut ignore = contents
            .lines()
            .map(|line: &str| line.trim().to_string())
            .filter(|line: &String| !line.is_empty() && !line.starts_with('#'))
            .collect::<Vec<String>>();

        ignore.push(String::from(".git"));
        Ignore {
            patterns: ignore,
        }
    }

    pub fn add_pattern(&mut self, pattern: &str) {
        self.patterns.push(pattern.to_string());
    }

    pub fn is_ignored(&self, path: &str) -> bool {
        for pattern in &self.patterns {
            if path.contains(pattern) {
                return true;
            }
        }
        false
    }
}