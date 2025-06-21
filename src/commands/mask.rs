// src/commands/mask.rs

//! Maszk‑teszt (pass/fail) vezérlés

use std::{error::Error, net::SocketAddr};

use crate::lxi::{query_scpi, send_scpi};

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() || cmd[0] != "mask" {
        return Ok(false);
    }
    if cmd.len() < 2 {
        help();
        return Ok(true);
    }

    match cmd[1].to_ascii_lowercase().as_str() {
        "on" | "off" => {
            send_scpi(addr, &format!(":MASK:ENAB {}", up(cmd[1].as_str()))).await?;
        }
        "run" | "start" => {
            send_scpi(addr, ":MASK:ENAB ON").await?;
            send_scpi(addr, ":MASK:OPER RUN").await?;
        }
        "stop" => send_scpi(addr, ":MASK:OPER STOP").await?,
        "source" if cmd.len() == 3 => {
            send_scpi(addr, &format!(":MASK:SOUR {}", cmd[2].to_ascii_uppercase())).await?;
        }
        "mdisp" | "stats" if cmd.len() == 3 => {
            send_scpi(addr, &format!(":MASK:MDIS {}", up(&cmd[2]))).await?;
        }
        "beep" | "sound" if cmd.len() == 3 => {
            send_scpi(addr, &format!(":MASK:OUTP {}", up(&cmd[2]))).await?;
        }
        "stopfail" if cmd.len() == 3 => {
            send_scpi(addr, &format!(":MASK:SOO {}", up(&cmd[2]))).await?;
        }
        "x" | "y" if cmd.len() == 3 => {
            let axis = if cmd[1] == "x" { "X" } else { "Y" };
            let val: f64 = cmd[2].parse()?;
            send_scpi(addr, &format!(":MASK:{axis} {}", val)).await?;
        }
        "create" => send_scpi(addr, ":MASK:CRE").await?,
        "reset"  => send_scpi(addr, ":MASK:RESet").await?,
        "results" => {
            let p = query_scpi(addr, ":MASK:PASS?").await?;
            let f = query_scpi(addr, ":MASK:FAIL?").await?;
            let t = query_scpi(addr, ":MASK:TOT?").await?;
            println!("Pass: {},  Fail: {},  Total: {}", p.trim(), f.trim(), t.trim());
        }
        _ => help(),
    }
    Ok(true)
}

fn up(s: &str) -> &'static str {
    if matches!(s.to_ascii_lowercase().as_str(), "on" | "1" | "true") { "ON" } else { "OFF" }
}

fn help() {
    eprintln!("mask <on|off|run|stop|source|x|y|stats|stopfail|beep|create|reset|results> …");
}
