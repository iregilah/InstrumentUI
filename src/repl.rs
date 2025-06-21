// src/repl.rs

//! Interaktív SCPI prompt (REPL) a műszerhez.

use std::{error::Error, net::SocketAddr};

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_lxi::LxiDevice;

/// Indítsd el a REPL‑t.  Ha a felhasználó `exit` vagy `quit`‑et ír,
/// a függvény visszatér.
pub async fn run_repl(addr: &SocketAddr) -> Result<(), Box<dyn Error>> {
    let mut dev = LxiDevice::connect(addr).await?;
    dev.send("*IDN?").await?;
    let idn = dev.receive().await?;
    println!("Connected: {}", idn.trim_end());
    println!("Type 'exit' to quit.");

    print!("SCPI> ");
    io::stdout().flush().await?;

    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await? {
        let cmd = line.trim();
        if cmd.eq_ignore_ascii_case("exit") || cmd.eq_ignore_ascii_case("quit") {
            break;
        }
        if cmd.is_empty() {
            print!("SCPI> ");
            io::stdout().flush().await?;
            continue;
        }

        dev.send(cmd).await?;
        if cmd.ends_with('?') {
            let resp = dev.receive().await?;
            println!("{}", resp.trim_end());
        } else {
            println!("(OK)");
        }

        print!("SCPI> ");
        io::stdout().flush().await?;
    }
    Ok(())
}
