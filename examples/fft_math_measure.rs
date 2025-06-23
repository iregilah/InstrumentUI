//! CH1→FFT, MATH‑szorzás CH1·CH2, majd gyors amplitude‑mérés.

/*
Mit látsz?
* FFT‑ablakban spektrum, majd visszavált valódi időtartományra és a
rózsaszín MATH‑trace jelenik meg.
 */

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

    println!("=== FFT + MATH DEMO ===");

    // -- FFT ------------------------------------------------------------
    send_scpi(&addr, ":MATH:DISP ON").await?;
    send_scpi(&addr, ":MATH:OPER FFT").await?;
    send_scpi(&addr, ":MATH:FFT:SOUR CHAN1").await?;
    send_scpi(&addr, ":MATH:FFT:WIND HANN").await?;
    println!(">> FFT – kék spektrum CH1‑ről,  Hanning ablak.");
    sleep(Duration::from_secs(3)).await;

    // -- MATH = CH1·CH2 --------------------------------------------------
    send_scpi(&addr, ":MATH:OPER MULT").await?;
    send_scpi(&addr, ":MATH:SOUR1 CHAN1").await?;
    send_scpi(&addr, ":MATH:SOUR2 CHAN2").await?;
    println!(">> MATH MULT – rózsaszín görbe megjelenik.");
    sleep(Duration::from_secs(3)).await;

    // -- Gyors csúcs‑csúcs mérés a MATH csatornán -----------------------
    let vpp = query_scpi(&addr, ":MEAS:ITEM? PKPK,MATH").await?;
    println!(">> MATH Pk‑Pk = {} V", vpp.trim());

    // -- FFT & MATH OFF --------------------------------------------------
    send_scpi(&addr, ":MATH:DISP OFF").await?;
    println!("=== DEMO END ===");
    Ok(())
}