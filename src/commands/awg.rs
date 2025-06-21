// src/commands/awg.rs

//! AWG (arbitrary / standard) vezérlés és waveform‑feltöltés.

use std::{error::Error, net::SocketAddr};

use crate::{
    io::{load_arb},
    lxi::{send_scpi},
};

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }

    /* -------------------- be/ki kapcsolás ----------------------------- */
    if cmd[0] == "awg" && cmd.len() >= 3 {
        let ch: u8 = cmd[1].parse()?;
        match cmd[2].to_ascii_lowercase().as_str() {
            "on"  => send_scpi(addr, &format!(":OUTPUT{ch} ON")).await?,
            "off" => send_scpi(addr, &format!(":OUTPUT{ch} OFF")).await?,
            wf    => {
                /* -- gyors‑preset hullámformák --------------------------- */
                let kind = match wf {
                    "sin" | "sine"   => "SIN",
                    "square" | "sq"  => "SQU",
                    "pulse"          => "PULS",
                    "ramp"           => "RAMP",
                    "noise"          => "NOIS",
                    "user"           => "USER",
                    _ => { eprintln!("Unknown waveform"); return Ok(true); }
                };

                let need = if kind == "NOIS" { 3 } else { 5 }; // paranccsal együtt
                if cmd.len() != need {
                    eprintln!("Usage error – lásd dokumentáció");
                    return Ok(true);
                }

                match kind {
                    "NOIS" => {
                        let amp:  f64 = cmd[3].parse()?;
                        let offs: f64 = cmd[4].parse()?;
                        send_scpi(addr, &format!(":SOUR{ch}:APPL:NOIS {amp},{offs}")).await?;
                    }
                    _ => {
                        let freq: f64 = cmd[3].parse()?;
                        let amp:  f64 = cmd[4].parse()?;
                        let offs: f64 = cmd[5].parse()?;
                        send_scpi(addr, &format!(":SOUR{ch}:APPL:{kind} {freq},{amp},{offs},0")).await?;
                    }
                }
                send_scpi(addr, &format!(":OUTPUT{ch} ON")).await?;
            }
        }
        return Ok(true);
    }

    /* -------------------- arb‑feltöltés ------------------------------- */
    if cmd[0] == "arb" && cmd.len() == 3 {
        let ch: u8 = cmd[1].parse()?;
        load_arb(addr, ch, &cmd[2]).await?;
        return Ok(true);
    }

    Ok(false)
}
