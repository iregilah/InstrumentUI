// src/commands/basic.rs

//! Alap‑vezérlő parancsok: futtatás, megállítás, skálák,
//! csatorna‑be‑/kikapcsolás, autoscale stb.

use std::{error::Error, net::SocketAddr};

use crate::io::parse_source_arg;
use crate::lxi::{query_scpi, send_scpi};

/// Feldolgoz – ha a parancs ebbe a csoportba tartozik, akkor
/// végrehajtja és `Ok(true)`‑val tér vissza, egyébként `Ok(false)`.
pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }

    match cmd[0].as_str() {
        /* ---- futás‑vezérlés ------------------------------------------- */
        "start"    => { send_scpi(addr, ":RUN").await?;           return Ok(true); }
        "stop"     => { send_scpi(addr, ":STOP").await?;          return Ok(true); }
        "single"   => { send_scpi(addr, ":SINGLE").await?;        return Ok(true); }
        "force"    => { send_scpi(addr, ":TFORCE").await?;        return Ok(true); }
        "auto"     => { send_scpi(addr, ":AUTOSCALE").await?;     return Ok(true); }

        /* ---- csatorna láthatóság -------------------------------------- */
        "ch" if cmd.len() == 3 => {
            let ch: u8 = cmd[1].parse()?;
            let on  = cmd[2].eq_ignore_ascii_case("on");
            send_scpi(addr, &format!(":CHAN{ch}:DISP {}", if on { "ON" } else { "OFF" })).await?;
            return Ok(true);
        }

        /* ---- vertikális skála, offset, coupling ----------------------- */
        "scale" if cmd.len() == 3 => {
            let ch: u8 = cmd[1].parse()?;
            let val: f64 = cmd[2].parse()?;
            send_scpi(addr, &format!(":CHAN{ch}:SCAL {}", val)).await?;
            return Ok(true);
        }
        "offset" if cmd.len() == 3 => {
            let ch: u8 = cmd[1].parse()?;
            let val: f64 = cmd[2].parse()?;
            send_scpi(addr, &format!(":CHAN{ch}:OFFS {}", val)).await?;
            return Ok(true);
        }
        "coupling" if cmd.len() == 3 => {
            let ch: u8 = cmd[1].parse()?;
            let mode = match cmd[2].to_ascii_lowercase().as_str() {
                "ac" => "AC", "dc" => "DC", "gnd" => "GND",
                _ => { eprintln!("Coupling must be AC|DC|GND"); return Ok(true); }
            };
            send_scpi(addr, &format!(":CHAN{ch}:COUP {}", mode)).await?;
            return Ok(true);
        }

        /* ---- időalap --------------------------------------------------- */
        "timebase" if cmd.len() == 2 => {
            let val: f64 = cmd[1].parse()?;
            send_scpi(addr, &format!(":TIM:SCAL {}", val)).await?;
            return Ok(true);
        }

        /* ---- memóriamélység ------------------------------------------- */
        "memdepth" if cmd.len() == 2 => {
            if cmd[1].eq_ignore_ascii_case("auto") {
                send_scpi(addr, ":ACQ:MDEP AUTO").await?;
            } else {
                let depth: u32 = cmd[1].parse()?;
                send_scpi(addr, &format!(":ACQ:MDEP {}", depth)).await?;
            }
            return Ok(true);
        }

        /* ---- egyszerű lekérdezés (query) ------------------------------ */
        "query" if cmd.len() >= 2 => {
            let scpi = cmd[1..].join(" ");
            let resp = query_scpi(addr, &scpi).await?;
            println!("{}", resp.trim_end());
            return Ok(true);
        }

        _ => {}
    }

    Ok(false)
}
