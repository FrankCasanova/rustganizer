#[cfg(test)]
mod tests {
    use rustganizer::platform::user::UserProvider;

    #[cfg(target_os = "macos")]
    use rustganizer::platform::user::MacUserProvider;
    #[cfg(all(unix, not(target_os = "macos")))]
    use rustganizer::platform::user::UnixUserProvider;
    #[cfg(target_os = "windows")]
    use rustganizer::platform::user::WindowsUserProvider;

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

    #[test]
    #[cfg(target_os = "macos")]
    fn test_list_users_macos() {
        let provider = MacUserProvider;
        let users = provider.list_users();
        // Should not panic and should return a Vec
        assert!(users.is_empty() || users.iter().all(|u| !u.is_empty()));
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_user_home_macos() {
        let provider = MacUserProvider;
        // Should return None for a non-existent user
        assert_eq!(provider.user_home("nonexistent_user_xyz"), None);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_user_home_existing_macos() {
        let provider = MacUserProvider;
        let users = provider.list_users();
        if let Some(user) = users.first() {
            let home = provider.user_home(user);
            assert!(home.is_some());
            let path = home.unwrap();
            assert!(path.exists());
        }
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn test_list_users_unix() {
        let provider = UnixUserProvider;
        let users = provider.list_users();
        // Should not panic and should return a Vec
        assert!(users.is_empty() || users.iter().all(|u| !u.is_empty()));
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn test_user_home_unix() {
        let provider = UnixUserProvider;
        // Should return None for a non-existent user
        assert_eq!(provider.user_home("nonexistent_user_xyz"), None);
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn test_user_home_existing_unix() {
        let provider = UnixUserProvider;
        let users = provider.list_users();
        if let Some(user) = users.first() {
            let home = provider.user_home(user);
            assert!(home.is_some());
            let path = home.unwrap();
            assert!(path.exists());
        }
    }
}
