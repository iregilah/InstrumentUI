//! Hozz létre egyszerű ablak‑maszkot, futtasd 5 s‑ig, majd STOP, statisztika.

use rigol_cli::lxi::{query_scpi, send_scpi};
use std::{net::SocketAddr, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::args()
        .skip_while(|s| s != "--ip")
        .nth(1)
        .unwrap_or("169.254.50.23:5555".into());
    let addr: SocketAddr = ip.parse()?;

    println!("=== MASK PASS/FAIL DEMO ===");

    // Egyszerű téglalap‑maszk a képernyő közepére
    send_scpi(&addr, ":MASK:CRE").await?;
    send_scpi(&addr, ":MASK:X 0.2").await?;   // bal
    send_scpi(&addr, ":MASK:Y 0.2").await?;   // felső
    send_scpi(&addr, ":MASK:X 0.8").await?;   // jobb
    send_scpi(&addr, ":MASK:Y 0.8").await?;   // alsó
    send_scpi(&addr, ":MASK:ENAB ON").await?;
    send_scpi(&addr, ":MASK:OPER RUN").await?;
    send_scpi(&addr, ":MASK:SOO ON").await?;  // stop on fail

    println!(">> Maszk aktív 5 s‑ig – PASS/FAIL számláló nő.");
    sleep(Duration::from_secs(5)).await;

    // Leállítás + eredmények
    send_scpi(&addr, ":MASK:OPER STOP").await?;
    let pass = query_scpi(&addr, ":MASK:PASS?").await?;
    let fail = query_scpi(&addr, ":MASK:FAIL?").await?;
    println!("==> PASS: {},  FAIL: {}", pass.trim(), fail.trim());

    // Takarítás
    send_scpi(&addr, ":MASK:ENAB OFF").await?;
    send_scpi(&addr, ":MASK:RESet").await?;

    println!("=== DEMO END ===");
    Ok(())
}
