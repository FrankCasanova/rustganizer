//! Logging and monitoring setup for Rustganizer

use tracing::{info, warn, error, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::path::PathBuf;
use crate::config::Config;

/// Initialize logging based on configuration
pub fn initialize_logging(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let log_level = match config.logging.level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true);

    let mut layers = vec![stdout_layer.boxed()];

    // Add file logging if configured
    if let Some(log_file) = &config.logging.file {
        let file_appender = tracing_appender::rolling::daily(
            log_file.parent().unwrap_or_else(|| PathBuf::from(".")),
            log_file.file_name().unwrap_or_default().to_string_lossy(),
        );
        
        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(file_appender)
            .with_ansi(false)
            .with_target(false)
            .with_thread_ids(true)
            .with_level(true);
            
        layers.push(file_layer.boxed());
    }

    tracing_subscriber::registry()
        .with(layers)
        .with(tracing_subscriber::filter::LevelFilter::from_level(log_level))
        .init();

    info!("Logging initialized at level: {}", config.logging.level);
    Ok(())
}

/// Log file organization operation start
pub fn log_organization_start(username: &str, lang: &str) {
    info!("Starting file organization for user: {} (language: {})", username, lang);
}

/// Log file organization completion
pub fn log_organization_complete(stats: &crate::organizer::types::FileStats, errors: &[String]) {
    info!("File organization completed: music={}, videos={}, images={}, docs={}", 
          stats.music, stats.videos, stats.images, stats.docs);
    
    if !errors.is_empty() {
        warn!("Organization completed with {} errors: {:?}", errors.len(), errors);
    }
}

/// Log file move operation
pub fn log_file_move(from: &std::path::Path, to: &std::path::Path, success: bool) {
    if success {
        debug!("Moved file from {:?} to {:?}", from, to);
    } else {
        error!("Failed to move file from {:?} to {:?}", from, to);
    }
}

/// Log directory analysis
pub fn log_directory_analysis(path: &std::path::Path, stats: &crate::organizer::types::FileStats) {
    debug!("Analyzed directory {:?}: music={}, videos={}, images={}, docs={}", 
           path, stats.music, stats.videos, stats.images, stats.docs);
}

/// Log configuration loading
pub fn log_config_load(config_path: &std::path::Path, success: bool) {
    if success {
        info!("Configuration loaded from: {:?}", config_path);
    } else {
        warn!("Failed to load configuration from: {:?}, using defaults", config_path);
    }
}

/// Log error with context
pub fn log_error_with_context(error: &crate::error::Error, context: &str) {
    error!("Error in {}: {}", context, error);
}

/// Performance logging
pub fn log_performance_metric(operation: &str, duration: std::time::Duration, item_count: usize) {
    let items_per_second = if duration.as_secs() > 0 {
        item_count as f64 / duration.as_secs_f64()
    } else {
        0.0
    };
    
    debug!("Performance - {}: {} items in {:.2}s ({:.2} items/s)", 
           operation, item_count, duration.as_secs_f64(), items_per_second);
}

/// Log user interaction
pub fn log_user_action(action: &str, details: &str) {
    info!("User action: {} - {}", action, details);
}

/// Log system information
pub fn log_system_info() {
    let cpu_count = num_cpus::get();
    let memory = match sysinfo::System::new_all().total_memory() {
        mem if mem > 0 => format!("{:.1} GB", mem as f64 / 1024.0 / 1024.0 / 1024.0),
        _ => "Unknown".to_string(),
    };
    
    info!("System info: {} CPUs, {} RAM", cpu_count, memory);
}