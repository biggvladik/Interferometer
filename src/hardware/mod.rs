// src/hardware/mod.rs
pub mod dll_wrapper;
pub mod types;

// Реэкспорт для удобства
pub use dll_wrapper::{USMCDLL, USMCController};
pub use types::*;