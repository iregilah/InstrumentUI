// src/commands/psu.rs
//! Power Supply (PSU) control commands
use std::{error::Error, net::SocketAddr};
use crate::lxi::{send_scpi, query_scpi};

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }
    if cmd[0] != "psu" {
        return Ok(false);
    }
    if cmd.len() < 2 {
        // Print usage if just "psu" with no further args
        eprintln!("psu <all> <on|off> | <channel> <on|off|volt|curr|measure ...>");
        return Ok(true);
    }
    // If second argument is "all", expecting on/off
    if cmd[1].eq_ignore_ascii_case("all") {
        if cmd.len() < 3 {
            eprintln!("psu all <on|off>");
        } else {
            let state = match cmd[2].to_ascii_lowercase().as_str() {
                "on" | "1" | "yes" | "true" => "ON",
                "off" | "0" | "no" | "false" => "OFF",
                _ => {
                    eprintln!("psu all <on|off>");
                    return Ok(true);
                }
            };
            send_scpi(addr, &format!(":OUTP ALL,{}", state)).await?;
        }
        return Ok(true);
    }
    // Otherwise, expect a channel number
    let ch: u8 = match cmd[1].parse() {
        Ok(n) => {
            if (1..=3).contains(&n) {
                n
            } else {
                eprintln!("Invalid channel (must be 1-3 or 'all')");
                return Ok(true);
            }
        }
        Err(_) => {
            eprintln!("psu <all> <on|off> | <channel> <on|off|volt|curr|measure ...>");
            return Ok(true);
        }
    };
    // Need at least one more subcommand after channel
    if cmd.len() < 3 {
        eprintln!("psu <channel> <on|off|volt|curr|measure ...>");
        return Ok(true);
    }
    match cmd[2].to_ascii_lowercase().as_str() {
        "on" | "1" | "yes" | "true" => {
            send_scpi(addr, &format!(":OUTP CH{},ON", ch)).await?;
        }
        "off" | "0" | "no" | "false" => {
            send_scpi(addr, &format!(":OUTP CH{},OFF", ch)).await?;
        }
        "volt" | "voltage" => {
            if cmd.len() < 4 {
                eprintln!("psu {} volt <value_V>", ch);
            } else {
                let val: f64 = cmd[3].parse()?;
                send_scpi(addr, &format!(":SOUR{}:VOLT {}", ch, val)).await?;
            }
        }
        "curr" | "current" => {
            if cmd.len() < 4 {
                eprintln!("psu {} curr <value_A>", ch);
            } else {
                let val: f64 = cmd[3].parse()?;
                send_scpi(addr, &format!(":SOUR{}:CURR {}", ch, val)).await?;
            }
        }
        "meas" | "measure" => {
            if cmd.len() < 4 {
                eprintln!("psu {} measure <volt|curr|power|all>", ch);
            } else {
                match cmd[3].to_ascii_lowercase().as_str() {
                    "volt" | "voltage" => {
                        let resp = query_scpi(addr, &format!("MEAS:VOLT? CH{}", ch)).await?;
                        println!("{}", resp.trim_end());
                    }
                    "curr" | "current" => {
                        let resp = query_scpi(addr, &format!("MEAS:CURR? CH{}", ch)).await?;
                        println!("{}", resp.trim_end());
                    }
                    "power" | "pow" => {
                        let resp = query_scpi(addr, &format!("MEAS:POWE? CH{}", ch)).await?;
                        println!("{}", resp.trim_end());
                    }
                    "all" => {
                        let v = query_scpi(addr, &format!("MEAS:VOLT? CH{}", ch)).await?;
                        let i = query_scpi(addr, &format!("MEAS:CURR? CH{}", ch)).await?;
                        let p = query_scpi(addr, &format!("MEAS:POWE? CH{}", ch)).await?;
                        println!("Voltage: {}, Current: {}, Power: {}",
                                 v.trim_end(), i.trim_end(), p.trim_end());
                    }
                    _ => {
                        eprintln!("psu {} measure <volt|curr|power|all>", ch);
                    }
                }
            }
        }
        _ => {
            eprintln!("psu <channel> <on|off|volt|curr|measure ...>");
        }
    }
    Ok(true)
}
