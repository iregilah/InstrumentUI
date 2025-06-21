// src/commands/measure.rs

//! Mérési és kijelző‑orientált parancsok (measure, counter, cursor).

use std::{error::Error, net::SocketAddr};

use crate::io::parse_source_arg;
use crate::lxi::{query_scpi, send_scpi};

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }

    /* ------------- single‑shot scalar mérés --------------------------- */
    if cmd[0] == "measure" && cmd.len() >= 3 {
        let item = cmd[1].to_ascii_uppercase();
        let s1   = parse_source_arg(&cmd[2])?;
        let scpi = if cmd.len() >= 4 {
            let s2 = parse_source_arg(&cmd[3])?;
            format!(":MEAS:ITEM? {item},{s1},{s2}")
        } else {
            format!(":MEAS:ITEM? {item},{s1}")
        };
        let resp = query_scpi(addr, &scpi).await?;
        println!("{}", resp.trim_end());
        return Ok(true);
    }

    /* ------------- beépített frekvenciaszámláló ----------------------- */
    if cmd[0] == "counter" {
        if cmd.len() == 1 {
            let v = query_scpi(addr, ":MEAS:COUN:VAL?").await?;
            println!("{}", v.trim_end());
        } else if cmd[1].eq_ignore_ascii_case("off") {
            send_scpi(addr, ":MEAS:COUN:SOUR OFF").await?;
        } else {
            let src = parse_source_arg(&cmd[1])?;
            send_scpi(addr, &format!(":MEAS:COUN:SOUR {}", src)).await?;
        }
        return Ok(true);
    }

    /* ------------- kurzor‑kezelés ------------------------------------- */
    if cmd[0] == "cursor" && cmd.len() >= 2 {
        match cmd[1].to_ascii_lowercase().as_str() {
            "mode" if cmd.len() == 3 => {
                let m = match cmd[2].to_ascii_lowercase().as_str() {
                    "off"   => "OFF",
                    "manual"=> "MANUAL",
                    "track" => "TRACK",
                    "auto"  => "AUTO",
                    "xy"    => "XY",
                    _ => { eprintln!("Unknown cursor mode"); return Ok(true); }
                };
                send_scpi(addr, &format!(":CURS:MODE {}", m)).await?;
            }
            "type" if cmd.len() == 3 => {
                let t = match cmd[2].to_ascii_lowercase().as_str() {
                    "x" => "X", "y" => "Y",
                    _ => { eprintln!("Type must be X|Y"); return Ok(true); }
                };
                send_scpi(addr, &format!(":CURS:MAN:TYPE {}", t)).await?;
            }
            "source"  if cmd.len() == 3 => {
                let s = parse_source_arg(&cmd[2])?;
                send_scpi(addr, &format!(":CURS:MAN:SOUR {}", s)).await?;
            }
            "ax" | "bx" | "ay" | "by" if cmd.len() == 3 => {
                let val: f64 = cmd[2].parse()?;
                let sym = cmd[1].to_ascii_uppercase();
                // mindkét al‑rendszert (MAN & TRAC) frissítjük
                send_scpi(addr, &format!(":CURS:MAN:{sym} {}", val)).await?;
                send_scpi(addr, &format!(":CURS:TRAC:{sym} {}", val)).await?;
            }
            _ => eprintln!("Unknown cursor sub‑command"),
        }
        return Ok(true);
    }

    Ok(false)
}
