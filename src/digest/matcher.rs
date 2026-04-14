use regex::Regex;

/// Matcher is used to check if the given file get to be a part of the final output
/// Checks the file path for the patterns provided as CLI options
pub struct Matcher {
    include_re: Regex,
    exclude_re: Regex,
}

impl Matcher {
    /// Creates a new instance of matcher
    /// Validates given RegEx
    /// If given RegEx is incorrect returns an error and exits the process
    pub fn new(include_pattern: &str, exclude_pattern: &str) -> Self {
        // Validate patterns
        let include_re = match Regex::new(include_pattern) {
            Ok(re) => re,
            Err(e) => {
                eprintln!("Failed to read match pattern\n{}", e);
                std::process::exit(1);
            }
        };
        let exclude_re = match Regex::new(exclude_pattern) {
            Ok(re) => re,
            Err(e) => {
                eprintln!("Failed to read exclude pattern\n{}", e);
                std::process::exit(1);
            }
        };

        Matcher {
            include_re,
            exclude_re,
        }
    }

    /// Check if the given string mathes the included regex
    /// and doesnt match the excluded
    pub fn is_match(&self, item: &str) -> bool {
        self.include_re.is_match(item) && !self.exclude_re.is_match(item)
    }
}
