// examples/psu_channel_on.rs
use rigol_cli::lxi::send_scpi;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "169.254.50.23:5555".into())
        .parse()?;

    let ch = 1;
    let volt = 5.0;
    let curr = 0.5;

    send_scpi(&addr, &format!(":SOUR{ch}:VOLT {volt}")).await?;
    send_scpi(&addr, &format!(":SOUR{ch}:CURR {curr}")).await?;
    send_scpi(&addr, &format!(":OUTP CH{ch},ON")).await?;
    println!("PSU CH{ch}: {volt} V / {curr} A, output ON");
    Ok(())
}
