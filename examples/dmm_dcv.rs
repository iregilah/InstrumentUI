// examples/dmm_dcv.rs
use rigol_cli::lxi::query_scpi;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "169.254.50.23:5555".into())
        .parse()?;

    let v = query_scpi(&addr, ":MEAS:VOLT:DC?").await?;
    println!("DC Voltage: {}", v.trim_end());
    Ok(())
}
