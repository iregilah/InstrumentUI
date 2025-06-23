//! Gyors AWG‑ellenőrzés – 1 kHz szinusz, majd zaj.

use rigol_cli::lxi::send_scpi;
use std::{net::SocketAddr, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::args()
        .skip_while(|s| s != "--ip")
        .nth(1)
        .unwrap_or("169.254.50.23:5555".into());
    let addr: SocketAddr = ip.parse()?;

    println!("=== AWG DEMO (CH1) ===");

    // 1 kHz, 1 Vpp szinusz
    send_scpi(&addr, ":SOURCE1:APPL:SIN 1e3,1,0,0").await?;
    send_scpi(&addr, ":OUTPUT1 ON").await?;
    println!(">> 1 kHz SIN – a kimeneten szinusz, csatornán ≈500 mV/div látható.");
    sleep(Duration::from_secs(2)).await;

    // Fehér zaj 500 mVpp
    send_scpi(&addr, ":SOURCE1:APPL:NOIS 0.5,0").await?;
    println!(">> NOISE – „vastag” zaj­szőnyeg jelenik meg.");
    sleep(Duration::from_secs(2)).await;

    // Kikapcsolás
    send_scpi(&addr, ":OUTPUT1 OFF").await?;
    println!(">> OFF – trace eltűnik (ha csak AWG‑ről ment).");

    println!("=== AWG DEMO END ===");
    Ok(())
}
