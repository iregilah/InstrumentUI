// src/commands/trigger.rs

//! Minden trigger‑típus („trig…”) összegyűjtve.

use std::{error::Error, net::SocketAddr};

use crate::{
    io::parse_source_arg,
    lxi::{query_scpi, send_scpi},
};

/// A fájl mérete miatt csak a leggyakrabban használt EDGE / LEVEL /
/// SLOPE / TIMEOUT és PULSE triggerek kerültek ide.  A további
/// különleges típusokat (RS232, I²C stb.) a következő iterációban
/// bővítjük.
pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() || !cmd[0].starts_with("trig") {
        return Ok(false);
    }

    /* ------------------- edge alapok ---------------------------------- */

    if cmd[0] == "trigsource" && cmd.len() == 2 {
        let src = parse_source_arg(&cmd[1])?;
        send_scpi(addr, ":TRIG:MODE EDGE").await?;
        send_scpi(addr, &format!(":TRIG:EDGE:SOUR {}", src)).await?;
        return Ok(true);
    }

    if cmd[0] == "triglevel" && cmd.len() == 3 {
        let src   = parse_source_arg(&cmd[1])?;
        let level = cmd[2].parse::<f64>()?;
        send_scpi(addr, ":TRIG:MODE EDGE").await?;
        send_scpi(addr, &format!(":TRIG:EDGE:SOUR {}", src)).await?;
        send_scpi(addr, &format!(":TRIG:EDGE:LEV {}",  level)).await?;
        return Ok(true);
    }

    if cmd[0] == "trigslope" && cmd.len() == 2 {
        let slope = match cmd[1].to_ascii_lowercase().as_str() {
            "pos" | "positive" | "rise" | "rising"   => "POS",
            "neg" | "negative" | "fall" | "falling"  => "NEG",
            _ => { eprintln!("Slope must be pos|neg"); return Ok(true); }
        };
        send_scpi(addr, ":TRIG:MODE EDGE").await?;
        send_scpi(addr, &format!(":TRIG:EDGE:SLOP {}", slope)).await?;
        return Ok(true);
    }

    /* ------------------- timeout -------------------------------------- */

    if cmd[0] == "trigtimeout" && cmd.len() == 4 {
        let src = parse_source_arg(&cmd[1])?;
        let slope = match cmd[2].to_ascii_lowercase().as_str() {
            "pos" | "positive" | "rise" | "rising"   => "POS",
            "neg" | "negative" | "fall" | "falling"  => "NEG",
            "either" | "both"                         => "RFAL",
            _ => { eprintln!("Edge must be pos|neg|either"); return Ok(true); }
        };
        let time = cmd[3].parse::<f64>()?;
        send_scpi(addr, ":TRIG:MODE TIM").await?;
        send_scpi(addr, &format!(":TRIG:TIM:SOUR {}", src)).await?;
        send_scpi(addr, &format!(":TRIG:TIM:SLOP {}", slope)).await?;
        send_scpi(addr, &format!(":TRIG:TIM:TIME {}", time)).await?;
        return Ok(true);
    }

    /* ------------------- pulse width ---------------------------------- */

    if cmd[0] == "trigpulse" && cmd.len() >= 5 {
        let src   = parse_source_arg(&cmd[1])?;
        let pol   = cmd[2].to_ascii_lowercase();
        let cond  = cmd[3].to_ascii_lowercase();
        let w1    = cmd[4].parse::<f64>()?;
        let w2    = if cond == "range" && cmd.len() >= 6 {
            Some(cmd[5].parse::<f64>()?)
        } else { None };

        let when = match (pol.as_str(), cond.as_str()) {
            ("pos", "less"   | "short")  => "LESS",
            ("pos", "greater"| "more")   => "GREA",
            ("pos", "range")             => "PGL",
            ("neg", "less"   | "short")  => "NLES",
            ("neg", "greater"| "more")   => "NGRE",
            ("neg", "range")             => "NGL",
            _ => { eprintln!("Invalid polarity/condition"); return Ok(true); }
        };

        send_scpi(addr, ":TRIG:MODE PULS").await?;
        send_scpi(addr, &format!(":TRIG:PULS:SOUR {}", src)).await?;
        send_scpi(addr, &format!(":TRIG:PULS:WHEN {}", when)).await?;
        if let Some(high) = w2 {
            send_scpi(addr, &format!(":TRIG:PULS:LWID {}", w1)).await?;
            send_scpi(addr, &format!(":TRIG:PULS:UWID {}", high)).await?;
        } else {
            send_scpi(addr, &format!(":TRIG:PULS:WIDT {}", w1)).await?;
        }
        return Ok(true);
    }

    /* ------------------- ismeretlen trig‑parancs ---------------------- */
    Ok(false)
}
