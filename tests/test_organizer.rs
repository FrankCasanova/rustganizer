#[cfg(test)]
mod tests {
    use rustganizer::config::Config;
    use rustganizer::organizer::mover::organize_files;

    #[test]
    fn test_organize_files_invalid_user_en() {
        let username = "nonexistent_user_xyz";
        let config = Config::default();
        let result = organize_files(username, "en", &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_organize_files_invalid_user_es() {
        let username = "nonexistent_user_xyz";
        let config = Config::default();
        let result = organize_files(username, "es", &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("no encontrado"));
    }

    // Add more tests for edge cases as needed
}
