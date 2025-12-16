// src/lib.rs

//! A könyvtár‐krate gyökere.
//! Itt deklaráljuk és publikáljuk az összes almodult, hogy a bináris
//! (`main.rs`) és egymás között is elérhessék őket.

pub mod utils;
pub mod lxi;
pub mod io;
pub mod commands;
pub mod repl;
pub mod cli;
pub mod prelude;
pub mod aggregator;
pub mod instrument;
pub mod graph_object;
pub mod heatmap_object;

/// Gyors eléréshez egy mini‑prelude:
pub use prelude::*;