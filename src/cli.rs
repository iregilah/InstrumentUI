// src/cli.rs

//! Egylövéses (nem interaktív) parancssori kezelés.
//!
//! **Figyelem:** a tényleges parancs‑implementációk a `commands`
//! alkönyvtárban vannak/lesznek.  Itt csak a belépési pont és a
//! *dispatcher* található.

use std::{error::Error, net::SocketAddr};

/// A felhasználó által megadott parancssor (pl. `["ch","1","on"]`)
/// végrehajtása.
///
/// A logika valójában a `commands::*` modulokban van; itt csak
/// továbbítjuk a hívást.
pub async fn run_cli(addr: &SocketAddr, cmd: &[String]) -> Result<(), Box<dyn Error>> {
    if cmd.is_empty() {
        eprintln!("Empty command");
        return Ok(());
    }
    crate::commands::dispatch(addr, cmd).await
}
