#[cfg(test)]
mod tests {
    use rustganizer::config::Config;
    use rustganizer::organizer::mover::organize_files;

    #[test]
    fn test_organize_files_empty_username() {
        let config = Config::default();
        let result = organize_files("", "en", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_organize_files_special_characters() {
        let config = Config::default();
        let result = organize_files("!@#$%^&*()", "en", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_organize_files_long_username() {
        let config = Config::default();
        let long_username = "a".repeat(256);
        let result = organize_files(&long_username, "en", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_organize_files_non_ascii_username() {
        let config = Config::default();
        let result = organize_files("测试用户", "en", &config);
        assert!(result.is_err());
    }
}
