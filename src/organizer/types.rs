// Structs and enums for organizer module.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct FileStats {
    pub music: usize,
    pub videos: usize,
    pub images: usize,
    pub docs: usize,
}

impl FileStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn add(&mut self, other: &FileStats) {
        self.music += other.music;
        self.videos += other.videos;
        self.images += other.images;
        self.docs += other.docs;
    }
}
