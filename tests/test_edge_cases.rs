#[cfg(test)]
mod tests {
    use rustganizer::organizer::mover::organize_files;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_organize_files_empty_username() {
        let result = organize_files("", "en");
        assert!(result.is_err());
    }

    #[test]
    fn test_organize_files_special_characters() {
        let result = organize_files("!@#$%^&*()", "en");
        assert!(result.is_err());
    }

    #[test]
    fn test_organize_files_long_username() {
        let long_username = "a".repeat(256);
        let result = organize_files(&long_username, "en");
        assert!(result.is_err());
    }

    #[test]
    fn test_organize_files_non_ascii_username() {
        let result = organize_files("测试用户", "en");
        assert!(result.is_err());
    }
}
