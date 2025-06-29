#[cfg(test)]
mod tests {
    #[cfg(target_os = "macos")]
    use rustganizer::platform::user::MacUserProvider;
    #[cfg(all(unix, not(target_os = "macos")))]
    use rustganizer::platform::user::UnixUserProvider;
    use rustganizer::platform::user::UserProvider;
    #[cfg(target_os = "windows")]
    use rustganizer::platform::user::WindowsUserProvider;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_provider_no_crash() {
        let provider = WindowsUserProvider;
        let _ = provider.list_users();
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_provider_no_crash() {
        let provider = MacUserProvider;
        let _ = provider.list_users();
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn test_unix_provider_no_crash() {
        let provider = UnixUserProvider;
        let _ = provider.list_users();
    }
}
