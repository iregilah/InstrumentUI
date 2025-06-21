// src/commands/display.rs

//! Képernyő‑beállítások: grid, fényerő, perzisztencia, törlés

use std::{error::Error, net::SocketAddr};

use crate::lxi::send_scpi;

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.len() < 2 || cmd[0] != "display" {
        return Ok(false);
    }

    match cmd[1].to_ascii_lowercase().as_str() {
        /* -------- perzisztencia (utánhúzás) --------------------------- */
        "persist" if cmd.len() == 3 => {
            if cmd[2].eq_ignore_ascii_case("off") {
                send_scpi(addr, ":DISP:GRAD:TIME 0").await?;
            } else {
                let t: f64 = cmd[2].parse()?;
                send_scpi(addr, &format!(":DISP:GRAD:TIME {}", t)).await?;
            }
        }

        /* -------- kirajzolási mód (pont / vonal) ---------------------- */
        "type" if cmd.len() == 3 => {
            let mode = match cmd[2].to_ascii_lowercase().as_str() {
                "dots" | "dot"          => "DOTS",
                "vect" | "vector" | "line" => "VECT",
                _ => { eprintln!("display type <dots|vectors>"); return Ok(true); }
            };
            send_scpi(addr, &format!(":DISP:TYPE {}", mode)).await?;
        }

        /* -------- globális fényerő ------------------------------------ */
        "bright" | "intensity" if cmd.len() == 3 => {
            let v: u8 = cmd[2].parse()?;
            if v > 100 { eprintln!("0‑100% intervallum."); }
            else { send_scpi(addr, &format!(":DISP:WBRI {}", v)).await?; }
        }

        /* -------- rács fényerő ---------------------------------------- */
        "gridbright" if cmd.len() == 3 => {
            let v: u8 = cmd[2].parse()?;
            if v > 100 { eprintln!("0‑100% intervallum."); }
            else { send_scpi(addr, &format!(":DISP:GBRI {}", v)).await?; }
        }

        /* -------- rács típusa ----------------------------------------- */
        "grid" if cmd.len() == 3 => {
            let g = match cmd[2].to_ascii_lowercase().as_str() {
                "full" => "FULL", "half" => "HALF", "none" => "NONE",
                _ => { eprintln!("display grid <full|half|none>"); return Ok(true); }
            };
            send_scpi(addr, &format!(":DISP:GRID {}", g)).await?;
        }

        /* -------- gyors törlés --------------------------------------- */
        "clear" if cmd.len() == 2 => {
            send_scpi(addr, ":DISP:CLE").await?;
        }

        _ => eprintln!("display <persist|type|bright|gridbright|grid|clear> …"),
    }
    Ok(true)
}
