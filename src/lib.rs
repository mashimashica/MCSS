//! MCSS (Multi-Agent Complex Social Simulation) Framework
//!
//! This is the main entry point for the MCSS framework.
//! It re-exports the kernel crate and provides any additional
//! high-level functionality specific to MCSS.

pub use kernel;

// 必要に応じて、MCSS固有の追加機能をここに実装します
// 例えば：
// pub mod mcss_specific_module;

/// Initializes the MCSS framework.
pub fn init() {
    println!("Initializing MCSS framework");
    // ここにMCSSの初期化ロジックを実装します
}