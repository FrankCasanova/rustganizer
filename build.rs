fn main() {
    // Re-run build script if these change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/crap.ico");
    println!("cargo:rerun-if-changed=resources/rustganizer.rc");

    // Only attempt to embed resources on Windows targets
    #[cfg(target_os = "windows")]
    {
        use std::path::PathBuf;

        // Resolve icon path relative to the crate root for reliability
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        let rc_path = PathBuf::from(&manifest_dir)
            .join("resources")
            .join("rustganizer.rc");

        // Compile the RC file which sets icon and version strings
        embed_resource::compile(
            rc_path.to_str().expect("rc path must be valid UTF-8"),
            embed_resource::NONE,
        );
    }
}
