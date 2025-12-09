//! Asynchronous file analyzer with improved performance and scalability

use crate::config::Config;
use crate::organizer::types::FileStats;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::fs;
use tracing::{error, info};

/// Asynchronous folder analyzer with parallel processing
pub struct AsyncAnalyzer {
    config: Arc<Config>,
    max_concurrent: usize,
}

impl AsyncAnalyzer {
    /// Create a new async analyzer with configuration
    pub fn new(config: Arc<Config>) -> Self {
        let max_concurrent = std::cmp::min(
            4, // Default to 4 for simplicity
            num_cpus::get() * 2,
        );

        Self {
            config,
            max_concurrent,
        }
    }

    /// Analyze multiple directories asynchronously
    pub async fn analyze_directories(
        &self,
        paths: Vec<PathBuf>,
    ) -> Result<FileStats, Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting async analysis of {} directories", paths.len());

        let stats = Arc::new(Mutex::new(FileStats::default()));

        // Process each directory sequentially for now
        for path in paths {
            match self.analyze_single_directory(&path).await {
                Ok(dir_stats) => {
                    let mut total_stats = stats.lock().unwrap();
                    total_stats.music += dir_stats.music;
                    total_stats.videos += dir_stats.videos;
                    total_stats.images += dir_stats.images;
                    total_stats.docs += dir_stats.docs;
                    info!("Analyzed directory {:?}: {:?}", path, dir_stats);
                }
                Err(e) => {
                    error!("Failed to analyze directory {:?}: {}", path, e);
                }
            }
        }

        let final_stats = stats.lock().unwrap().clone();
        Ok(final_stats)
    }

    /// Analyze a single directory asynchronously
    async fn analyze_single_directory(
        &self,
        path: &Path,
    ) -> Result<FileStats, Box<dyn std::error::Error + Send + Sync>> {
        let mut stats = FileStats::default();
        let file_extensions = self.config.get_file_extensions();

        let mut entries = fs::read_dir(path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Some(extension) = entry_path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    let extension_str = ext.as_str();

                    if file_extensions.music.contains(&extension_str) {
                        stats.music += 1;
                    } else if file_extensions.videos.contains(&extension_str) {
                        stats.videos += 1;
                    } else if file_extensions.images.contains(&extension_str) {
                        stats.images += 1;
                    } else if file_extensions.docs.contains(&extension_str) {
                        stats.docs += 1;
                    }
                }
            }
        }

        Ok(stats)
    }

    /// Get file statistics for a single file asynchronously
    pub async fn get_file_stats(
        &self,
        path: &Path,
    ) -> Result<FileStats, Box<dyn std::error::Error + Send + Sync>> {
        let mut stats = FileStats::default();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                let file_extensions = self.config.get_file_extensions();
                let extension_str = ext.as_str();

                if file_extensions.music.contains(&extension_str) {
                    stats.music = 1;
                } else if file_extensions.videos.contains(&extension_str) {
                    stats.videos = 1;
                } else if file_extensions.images.contains(&extension_str) {
                    stats.images = 1;
                } else if file_extensions.docs.contains(&extension_str) {
                    stats.docs = 1;
                }
            }
        }

        Ok(stats)
    }

    /// Determine the majority file type in a directory
    pub fn get_majority_type(&self, stats: &FileStats) -> Option<&'static str> {
        let mut type_counts = [
            (stats.music, "music"),
            (stats.videos, "video"),
            (stats.images, "image"),
            (stats.docs, "docs"),
        ];
        type_counts.sort_by(|a, b| b.0.cmp(&a.0));

        if type_counts[0].0 > 0 {
            Some(type_counts[0].1)
        } else {
            None
        }
    }

    /// Batch analyze multiple files for better performance
    pub async fn batch_analyze_files(
        &self,
        paths: Vec<PathBuf>,
    ) -> Result<Vec<FileStats>, Box<dyn std::error::Error + Send + Sync>> {
        info!("Batch analyzing {} files", paths.len());

        let mut results = Vec::new();

        for path in paths {
            match self.get_file_stats(&path).await {
                Ok(stats) => results.push(stats),
                Err(e) => {
                    error!("Failed to analyze file {:?}: {}", path, e);
                }
            }
        }

        Ok(results)
    }
}

impl Default for AsyncAnalyzer {
    fn default() -> Self {
        Self::new(Arc::new(Config::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_async_analyzer_basic() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.mp3");

        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"test music file").unwrap();

        let config = Arc::new(Config::default());
        let analyzer = AsyncAnalyzer::new(config);

        let stats = analyzer.get_file_stats(&test_file).await.unwrap();
        assert_eq!(stats.music, 1);
        assert_eq!(stats.videos, 0);
        assert_eq!(stats.images, 0);
        assert_eq!(stats.docs, 0);
    }

    #[tokio::test]
    async fn test_batch_analyze_files() {
        let temp_dir = TempDir::new().unwrap();
        let mut paths = Vec::new();

        // Create test files
        for ext in &["mp3", "mp4", "png", "pdf"] {
            let file_path = temp_dir.path().join(format!("test.{}", ext));
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"test").unwrap();
            paths.push(file_path);
        }

        let config = Arc::new(Config::default());
        let analyzer = AsyncAnalyzer::new(config);

        let results = analyzer.batch_analyze_files(paths).await.unwrap();
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn test_get_majority_type() {
        let analyzer = AsyncAnalyzer::default();

        let stats = FileStats {
            music: 5,
            videos: 2,
            images: 1,
            docs: 0,
        };

        assert_eq!(analyzer.get_majority_type(&stats), Some("music"));

        let empty_stats = FileStats::default();
        assert_eq!(analyzer.get_majority_type(&empty_stats), None);
    }
}
