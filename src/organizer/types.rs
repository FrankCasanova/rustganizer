// Structs and enums for organizer module.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStats {
    pub music: usize,
    pub videos: usize,
    pub images: usize,
    pub docs: usize,
}
