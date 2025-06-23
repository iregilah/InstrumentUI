//! Trigger‑beállítások gyors szemrevételezéséhez.

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

    println!("=== TRIGGER DEMO ===");

    // Edge trigger, CH1, pozitív él, 0 V szint
    send_scpi(&addr, ":TRIG:MODE EDGE").await?;
    send_scpi(&addr, ":TRIG:EDGE:SOUR CHAN1").await?;
    send_scpi(&addr, ":TRIG:EDGE:SLOP POS").await?;
    send_scpi(&addr, ":TRIG:EDGE:LEV 0").await?;
    println!(">> Pozitív él‑trigger 0 V‑on  – sárga ▼ szimbólum + ↑ ikon");
    sleep(Duration::from_secs(2)).await;

    // Negatív élre váltunk
    send_scpi(&addr, ":TRIG:EDGE:SLOP NEG").await?;
    println!(">> Negatív él‑trigger – ikon ▼ lefelé mutat.");
    sleep(Duration::from_secs(2)).await;

    // Timeout‑trigger 5 ms, CH1, bármely él
    send_scpi(&addr, ":TRIG:MODE TIM").await?;
    send_scpi(&addr, ":TRIG:TIM:SOUR CHAN1").await?;
    send_scpi(&addr, ":TRIG:TIM:SLOP RFAL").await?;
    send_scpi(&addr, ":TRIG:TIM:TIME 0.005").await?;
    println!(">> Timeout 5 ms – a Trigger indikátor T ikonra vált.");
    sleep(Duration::from_secs(2)).await;

    println!("=== TRIGGER DEMO END ===");
    Ok(())
}
