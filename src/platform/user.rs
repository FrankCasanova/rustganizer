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
        use std::path::Path;
        use std::process::Command;
        let output = Command::new("cmd").args(["/C", "net user"]).output();
        let mut valid_users = Vec::new();
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut users = Vec::new();
            let mut in_users = false;
            for line in stdout.lines() {
                if line.contains("---") {
                    in_users = !in_users;
                    continue;
                }
                if in_users {
                    users.extend(line.split_whitespace().map(|s| s.to_string()));
                }
            }
            let user_dirs = ["C:/Users", "C:/Usuarios"];
            for user in users {
                if user.eq_ignore_ascii_case("the")
                    || user.eq_ignore_ascii_case("command")
                    || user.eq_ignore_ascii_case("completed")
                    || user.eq_ignore_ascii_case("successfully.")
                {
                    continue;
                }
                let mut found = false;
                for base in &user_dirs {
                    let path = Path::new(base).join(&user);
                    if path.exists() {
                        found = true;
                        break;
                    }
                }
                if found {
                    valid_users.push(user);
                }
            }
        }
        valid_users
    }
    fn user_home(&self, username: &str) -> Option<PathBuf> {
        let user_dirs = ["C:/Users", "C:/Usuarios"];
        for base in &user_dirs {
            let candidate = PathBuf::from(base).join(username);
            if candidate.exists() {
                return Some(candidate);
            }
        }
        None
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
