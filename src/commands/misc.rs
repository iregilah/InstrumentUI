// src/commands/misc.rs

//! Egyéb kisebb kényelmi parancsok: reset, hiba‑lista, hang, skála stb.

use std::{error::Error, net::SocketAddr};

use crate::{
    lxi::{send_scpi, query_scpi},
};

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }

    match cmd[0].as_str() {
        /* --------‑ teljes reset -------------------------------------- */
        "reset" => {
            send_scpi(addr, "*RST").await?;
            return Ok(true);
        }

        /* --------‑ hibatároló kiolvasása ----------------------------- */
        "error" => {
            loop {
                let e = query_scpi(addr, ":SYST:ERR?").await?;
                let t = e.trim();
                if t.starts_with('0') { break; }
                println!("{t}");
            }
            return Ok(true);
        }

        /* --------‑ beep ki‑/bekapcsolás ------------------------------ */
        "beep" if cmd.len() == 2 => {
            send_scpi(addr, &format!(":SYST:BEEP {}", yesno(cmd[1].as_str()))).await?;
            return Ok(true);
        }

        /* --------‑ csatorna‑opciók  --------------------------------- */
        "units" | "bwlimit" | "probe" | "range" | "invert" if cmd.len() == 3 => {
            let ch: u8 = cmd[1].parse()?;
            let scpi = match cmd[0] {
                "units"   => {
                    let u = match cmd[2].to_ascii_lowercase().as_str() {
                        "v" | "volt"        => "VOLT",
                        "a" | "amp"         => "AMP",
                        "w" | "watt"        => "WATT",
                        _ => { eprintln!("units <volt|amp|watt>"); return Ok(true); }
                    };
                    format!(":CHAN{ch}:UNIT {u}")
                }
                "bwlimit" => format!(":CHAN{ch}:BWLimit {}", yesno(cmd[2].as_str())),
                "probe"   => format!(":CHAN{ch}:PROB {}", cmd[2]),
                "range"   => format!(":CHAN{ch}:RANG {}", cmd[2]),
                "invert"  => format!(":CHAN{ch}:INV {}", yesno(cmd[2].as_str())),
                _ => unreachable!(),
            };
            send_scpi(addr, &scpi).await?;
            return Ok(true);
        }

        _ => {}
    }

    Ok(false)
}

/* --------------------------------------------------------------------- */

fn yesno(s: &str) -> &'static str {
    if matches!(s.to_ascii_lowercase().as_str(), "on" | "1" | "yes" | "true") { "ON" } else { "OFF" }
}
