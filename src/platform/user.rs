use std::path::PathBuf;

pub trait UserProvider {
    /// Returns a list of usernames available on the system.
    fn list_users(&self) -> Vec<String>;
    /// Returns the home directory for a given username, or None if not found.
    fn user_home(&self, username: &str) -> Option<PathBuf>;
}

#[cfg(target_os = "windows")]
pub struct WindowsUserProvider;

#[cfg(target_os = "windows")]
impl UserProvider for WindowsUserProvider {
    fn list_users(&self) -> Vec<String> {
        let mut users = Vec::new();
        if let Ok(entries) = std::fs::read_dir("C:/Users") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        users.push(name.to_string());
                    }
                }
            }
        }
        users
    }
    fn user_home(&self, username: &str) -> Option<PathBuf> {
        let candidate = PathBuf::from("C:/Users").join(username);
        if candidate.exists() {
            Some(candidate)
        } else {
            None
        }
    }
}

#[cfg(target_os = "macos")]
pub struct MacUserProvider;

#[cfg(target_os = "macos")]
impl UserProvider for MacUserProvider {
    fn list_users(&self) -> Vec<String> {
        let mut users = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/Users") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        users.push(name.to_string());
                    }
                }
            }
        }
        users
    }
    fn user_home(&self, username: &str) -> Option<PathBuf> {
        let candidate = PathBuf::from("/Users").join(username);
        if candidate.exists() {
            Some(candidate)
        } else {
            None
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
pub struct UnixUserProvider;

#[cfg(all(unix, not(target_os = "macos")))]
impl UserProvider for UnixUserProvider {
    fn list_users(&self) -> Vec<String> {
        let mut users = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/home") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        users.push(name.to_string());
                    }
                }
            }
        }
        users
    }
    fn user_home(&self, username: &str) -> Option<PathBuf> {
        let candidate = PathBuf::from("/home").join(username);
        if candidate.exists() {
            Some(candidate)
        } else {
            None
        }
    }
}
