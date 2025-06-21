// src/commands/acquire.rs

//! Mintavételezés / memóriamélység / frame‑record funkciók

use std::{error::Error, net::SocketAddr};

use crate::{
    lxi::send_scpi,
    io::{save_config, load_config},   // kényelmi áthívások
};

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }

    /* ---------------- mintavételezési típus ------------------------- */
    if cmd[0] == "acquire" && cmd.len() >= 2 {
        match cmd[1].to_ascii_lowercase().as_str() {
            "normal"           => send_scpi(addr, ":ACQ:TYPE NORM").await?,
            "peak" | "peakdet" => send_scpi(addr, ":ACQ:TYPE PEAK").await?,
            "hres" | "hires"   => send_scpi(addr, ":ACQ:TYPE HRES").await?,
            "avg" | "average"  => {
                send_scpi(addr, ":ACQ:TYPE AVER").await?;
                if let Some(n) = cmd.get(2) {
                    let k: u16 = n.parse()?;
                    send_scpi(addr, &format!(":ACQ:AVER {}", k)).await?;
                }
            }
            _ => eprintln!("acquire <normal|peak|hres|avg> [count]"),
        }
        return Ok(true);
    }

    /* ---------------- memóriamélység ------------------------------- */
    if cmd[0] == "memdepth" && cmd.len() == 2 {
        if cmd[1].eq_ignore_ascii_case("auto") {
            send_scpi(addr, ":ACQ:MDEP AUTO").await?;
        } else {
            let d: u32 = cmd[1].parse()?;
            send_scpi(addr, &format!(":ACQ:MDEP {}", d)).await?;
        }
        return Ok(true);
    }

    /* ---------------- waveform‑record / replay ---------------------- */
    if cmd[0] == "record" && cmd.len() >= 2 {
        match cmd[1].to_ascii_lowercase().as_str() {
            "start" => {
                let frames = cmd.get(2)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(1);
                send_scpi(addr, &format!(":FUNC:WREC:FEND {}", frames)).await?;
                send_scpi(addr, ":FUNC:WREC:ENAB ON").await?;
            }
            "stop"  => send_scpi(addr, ":FUNC:WREC:ENAB OFF").await?,
            "frame" if cmd.len() == 3 => {
                let f: u32 = cmd[2].parse()?;
                send_scpi(addr, ":FUNC:WREP:OPER STOP").await.ok();
                send_scpi(addr, &format!(":FUNC:WREP:FCURR {}", f)).await?;
            }
            "play"  => {
                send_scpi(addr, ":FUNC:WREP:MODE REPEAT").await?;
                send_scpi(addr, ":FUNC:WREP:OPER PLAY").await?;
            }
            _ => eprintln!("record <start [N]|stop|frame N|play>"),
        }
        return Ok(true);
    }

    /* ---------------- setup mentés / visszatöltés ------------------- */
    match cmd[0].as_str() {
        "savecfg" if cmd.len() == 2 => { save_config(addr, &cmd[1]).await?; return Ok(true); }
        "loadcfg" if cmd.len() == 2 => { load_config(addr, &cmd[1]).await?; return Ok(true); }
        _ => {}
    }

    Ok(false)
}
