use std::collections::HashMap;

/// Configuration struct to hold file organization rules and localized directory names
#[derive(Debug, Clone)]
pub struct Config {
    pub file_extensions: FileExtensions,
    pub localized_dirs: HashMap<String, HashMap<String, String>>,
    pub error_messages: HashMap<String, ErrorMessages>,
}

/// File extension mappings for different categories
#[derive(Debug, Clone)]
pub struct FileExtensions {
    pub music: Vec<&'static str>,
    pub videos: Vec<&'static str>,
    pub images: Vec<&'static str>,
    pub docs: Vec<&'static str>,
}

/// Localized error messages
#[derive(Debug, Clone)]
pub struct ErrorMessages {
    pub empty_username: &'static str,
    pub user_not_found: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        let mut localized_dirs = HashMap::new();
        
        // English directory names
        let english_dirs = HashMap::from([
            ("Downloads".to_string(), "Downloads".to_string()),
            ("Desktop".to_string(), "Desktop".to_string()),
            ("Music".to_string(), "Music".to_string()),
            ("Videos".to_string(), "Videos".to_string()),
            ("Pictures".to_string(), "Pictures".to_string()),
            ("Documents".to_string(), "Documents".to_string()),
        ]);
        localized_dirs.insert("en".to_string(), english_dirs);
        
        // Spanish directory names
        let spanish_dirs = HashMap::from([
            ("Downloads".to_string(), "Descargas".to_string()),
            ("Desktop".to_string(), "Escritorio".to_string()),
            ("Music".to_string(), "Música".to_string()),
            ("Videos".to_string(), "Vídeos".to_string()),
            ("Pictures".to_string(), "Imágenes".to_string()),
            ("Documents".to_string(), "Documentos".to_string()),
        ]);
        localized_dirs.insert("es".to_string(), spanish_dirs);

        let mut error_messages = HashMap::new();
        error_messages.insert("en".to_string(), ErrorMessages {
            empty_username: "Empty username. Please enter a valid username.",
            user_not_found: "User {username} not found. Please enter a valid username.",
        });
        
        error_messages.insert("es".to_string(), ErrorMessages {
            empty_username: "Usuario vacío. Por favor, ingrese un nombre de usuario válido.",
            user_not_found: "Usuario {username} no encontrado. Por favor, ingrese un nombre de usuario válido.",
        });

        Config {
            file_extensions: FileExtensions {
                music: vec!["mp3", "ogg", "wav", "flac"],
                videos: vec!["mp4", "avi", "mkv", "mov"],
                images: vec!["png", "jpg", "jpeg", "gif"],
                docs: vec!["pdf", "txt", "epub"],
            },
            localized_dirs,
            error_messages,
        }
    }
}

impl Config {
    /// Get localized directory name for given language and logical name
    pub fn get_localized_dir(&self, lang: &str, logical_name: &str) -> String {
        self.localized_dirs
            .get(lang)
            .and_then(|dirs| dirs.get(logical_name).cloned())
            .unwrap_or_else(|| logical_name.to_string())
    }

    /// Get file extension mappings
    pub fn get_file_extensions(&self) -> &FileExtensions {
        &self.file_extensions
    }

    /// Get error message for given language and key
    pub fn get_error_message(&self, lang: &str, key: &str, username: &str) -> String {
        if let Some(error_msgs) = self.error_messages.get(lang) {
            let message = match key {
                "empty_username" => error_msgs.empty_username,
                "user_not_found" => error_msgs.user_not_found,
                _ => "",
            };
            
            if !message.is_empty() {
                return message.replace("{username}", username);
            }
        }
        
        // Fallback to English
        let fallback = match key {
            "empty_username" => "Empty username. Please enter a valid username.",
            "user_not_found" => "User {username} not found. Please enter a valid username.",
            _ => "Unknown error",
        };
        fallback.replace("{username}", username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        // Test English directories
        let english_dirs = config.get_localized_dir("en", "Downloads");
        assert_eq!(english_dirs, "Downloads");
        
        let spanish_dirs = config.get_localized_dir("es", "Downloads");
        assert_eq!(spanish_dirs, "Descargas");
    }

    #[test]
    fn test_file_extensions() {
        let config = Config::default();
        let extensions = config.get_file_extensions();
        
        assert!(extensions.music.contains(&"mp3"));
        assert!(extensions.videos.contains(&"mp4"));
        assert!(extensions.images.contains(&"png"));
        assert!(extensions.docs.contains(&"pdf"));
    }

    #[test]
    fn test_error_messages() {
        let config = Config::default();
        
        let empty_msg = config.get_error_message("en", "empty_username", "testuser");
        assert_eq!(empty_msg, "Empty username. Please enter a valid username.");
        
        let not_found_msg = config.get_error_message("es", "user_not_found", "testuser");
        assert_eq!(not_found_msg, "Usuario testuser no encontrado. Por favor, ingrese un nombre de usuario válido.");
    }
}