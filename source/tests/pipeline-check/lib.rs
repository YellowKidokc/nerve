#![allow(dead_code)]
mod config {
    // The console's configuration contract; no desktop/WebView dependencies needed.
    pub struct PipelineConfig {
        pub enabled: bool,
        pub roots: Vec<String>,
        pub python_exe: String,
    }
}
#[path = "../../src/pipeline.rs"]
mod pipeline;
