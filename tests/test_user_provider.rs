#[cfg(test)]
mod tests {
    use rustganizer::platform::user::{UserProvider, WindowsUserProvider};
    use std::path::PathBuf;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_list_users_windows() {
        let provider = WindowsUserProvider;
        let users = provider.list_users();
        // Should not panic and should return a Vec
        assert!(users.is_empty() || users.iter().all(|u| !u.is_empty()));
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_user_home_windows() {
        let provider = WindowsUserProvider;
        // Should return None for a non-existent user
        assert_eq!(provider.user_home("nonexistent_user_xyz"), None);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_user_home_existing_windows() {
        let provider = WindowsUserProvider;
        let users = provider.list_users();
        if let Some(user) = users.first() {
            let home = provider.user_home(user);
            assert!(home.is_some());
            let path = home.unwrap();
            assert!(path.exists());
        }
    }
}
