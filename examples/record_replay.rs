//! 10 frame rögzítés, majd visszajátszás „movie” módban.

use rigol_cli::lxi::send_scpi;
use std::{net::SocketAddr, time::Duration};
use tokio::time::sleep;
/*
Látható eredmény
* >REC< piktogram rögzítéskor piros, utána „▶” ikon és a
képernyő keretében futó frame‑számláló.


 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::args()
        .skip_while(|s| s != "--ip")
        .nth(1)
        .unwrap_or("169.254.50.23:5555".into());
    let addr: SocketAddr = ip.parse()?;

    println!("=== REC/REPLAY DEMO ===");

    // Egyszerű beállítások: normál akvizíció, 2 ms/div
    send_scpi(&addr, ":ACQ:TYPE NORM").await?;
    send_scpi(&addr, ":TIM:SCAL 0.002").await?;

    // -- 10 frame rögzítése --------------------------------------------
    send_scpi(&addr, ":FUNC:WREC:FEND 10").await?;
    send_scpi(&addr, ":FUNC:WREC:ENAB ON").await?;
    println!(">> RECORD 10 frame – a REC piktogram pirosra vált.");
    sleep(Duration::from_secs(3)).await;

    send_scpi(&addr, ":FUNC:WREC:ENAB OFF").await?;
    println!(">> Recording stopt.");

    // -- visszajátszás „film‑módban” -----------------------------------
    send_scpi(&addr, ":FUNC:WREP:MODE REPEAT").await?;
    send_scpi(&addr, ":FUNC:WREP:OPER PLAY").await?;
    println!(">> PLAY – a képernyőn folyamatosan váltogatódik a 10 frame.");
    sleep(Duration::from_secs(4)).await;

    // -- leállítás ------------------------------------------------------
    send_scpi(&addr, ":FUNC:WREP:OPER STOP").await?;
    println!("=== DEMO END ===");
    Ok(())
}
