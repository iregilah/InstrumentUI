//! I²C dekóder bekapcs, CH1=SCL  CH2=SDA, 7‑bites címzés

/*
Mit figyelj?
I²C START/STOP nyilak, hexadecimális byte‑“buborékok”, ACK/‑‑ jelölés.
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

    println!("=== I2C DECODE DEMO ===");

    // Digitális csatornák KI, analógra támaszkodunk
    send_scpi(&addr, ":CHAN1:DISP ON").await?;
    send_scpi(&addr, ":CHAN2:DISP ON").await?;

    // Dekóder CH1‑re/CH2‑re
    send_scpi(&addr, ":DEC1:MODE IIC").await?;
    send_scpi(&addr, ":DEC1:DISP ON").await?;
    send_scpi(&addr, ":DEC1:IIC:CLK CHAN1").await?;
    send_scpi(&addr, ":DEC1:IIC:DATA CHAN2").await?;
    send_scpi(&addr, ":DEC1:IIC:ADDR 7BIT").await?;

    println!(">> Ha I²C‑forgalom van, lila cím‑/adatbuborékok jelennek meg.");
    sleep(Duration::from_secs(6)).await;

    // Dekóder OFF
    send_scpi(&addr, ":DEC1:DISP OFF").await?;
    println!("=== DEMO END ===");
    Ok(())
}
