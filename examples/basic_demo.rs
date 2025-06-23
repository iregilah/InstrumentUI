//! Egyszerű láthatóság / skála / autoscale próbák.

use rigol_cli::{
    lxi::send_scpi,
};
use std::{net::SocketAddr, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---- IP átvehető --ip flaggel ------------------------------------
    let mut ip = std::env::args()
        .skip_while(|s| s != "--ip")
        .nth(1)
        .unwrap_or("169.254.50.23:5555".into());

    let addr: SocketAddr = ip.parse()?;

    println!("=== BASIC DEMO on {addr} ===");

    // 1. CH1 KI
    send_scpi(&addr, ":CHAN1:DISP OFF").await?;
    println!(">> CH1 OFF  – A nyomvonal el kell hogy tűnjön.");
    sleep(Duration::from_secs(1)).await;

    // 2. CH1 BE
    send_scpi(&addr, ":CHAN1:DISP ON").await?;
    println!(">> CH1 ON   – Nyomvonal visszatér.");
    sleep(Duration::from_secs(1)).await;

    // 3. Vertikális skála 500 mV/div
    send_scpi(&addr, ":CHAN1:SCAL 0.5").await?;
    println!(">> CH1 SCALE 0.5 V/div – Y‑skála összeugrik.");
    sleep(Duration::from_secs(1)).await;

    // 4. Autoscale
    send_scpi(&addr, ":AUTOSCALE").await?;
    println!(">> AUTOSCALE – A scope újra­beállítja a skálákat.");
    sleep(Duration::from_secs(2)).await;

    println!("=== BASIC DEMO END ===");
    Ok(())
}