//! Bekapcsolja az LA‑modult, 1 V küszöb, majd sorban lekapcsolja a biteket.

/*
Elvárt vizuális jelek

* Megjelenik a 16 digitális sáv, majd a felső 8 eltűnik,
elő‑/utána a trigger menüben is kevesebb csatorna listázódik.
 */

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

    println!("=== LOGIC ANALYZER DEMO ===");

    // LA be, mind a 16 csatorna látható
    send_scpi(&addr, ":LA:STAT ON").await?;
    println!(">> LA ON – színes D0…D15 jelek a képernyő alján.");
    sleep(Duration::from_secs(1)).await;

    // 1 V küszöb mindkét POD‑on
    send_scpi(&addr, ":LA:POD1:THR 1").await?;
    send_scpi(&addr, ":LA:POD2:THR 1").await?;
    println!(">> Threshold 1 V – trigger‑/dekóder‑szinten is frissül.");
    sleep(Duration::from_secs(1)).await;

    // Sorban kikapcsoljuk D8…D15‑öt
    for d in 8..16 {
        send_scpi(&addr, &format!(":LA:DIG{d}:DISP OFF")).await?;
        println!("   D{d} OFF");
        sleep(Duration::from_millis(400)).await;
    }

    // LA modul kikapcs
    send_scpi(&addr, ":LA:STAT OFF").await?;
    println!("=== DEMO END ===");
    Ok(())
}
