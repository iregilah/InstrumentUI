// examples/sine_1khz.rs
use rigol_cli::lxi::send_scpi;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "169.254.50.23:5555".into())
        .parse()?;

    let freq = 1_000.0;
    let vpp = 2.0;
    send_scpi(&addr, &format!(":SOUR1:APPL:SIN {freq},{vpp},0,0")).await?;
    send_scpi(&addr, ":OUTPUT1 ON").await?;
    println!("AWG OUT1: 1 kHz sine, {vpp} Vpp");
    Ok(())
}
