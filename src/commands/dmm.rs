// src/commands/dmm.rs
//! Digital Multimeter (DMM) measurement commands
use std::{error::Error, net::SocketAddr};
use crate::lxi::query_scpi;

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }
    if cmd[0] != "dmm" {
        return Ok(false);
    }
    // If only "dmm" with no additional args, print usage
    if cmd.len() < 2 {
        eprintln!("dmm <dcv|acv|dci|aci|res|fres|freq|cont|diode|cap>");
        return Ok(true);
    }
    match cmd[1].to_ascii_lowercase().as_str() {
        "dcv" | "volt_dc" | "vdc" => {
            let resp = query_scpi(addr, ":MEAS:VOLT:DC?").await?;
            println!("{}", resp.trim_end());
        }
        "acv" | "volt_ac" | "vac" => {
            let resp = query_scpi(addr, ":MEAS:VOLT:AC?").await?;
            println!("{}", resp.trim_end());
        }
        "dci" | "curr_dc" | "idc" => {
            let resp = query_scpi(addr, ":MEAS:CURR:DC?").await?;
            println!("{}", resp.trim_end());
        }
        "aci" | "curr_ac" | "iac" => {
            let resp = query_scpi(addr, ":MEAS:CURR:AC?").await?;
            println!("{}", resp.trim_end());
        }
        "res" | "ohm" => {
            let resp = query_scpi(addr, ":MEAS:RES?").await?;
            println!("{}", resp.trim_end());
        }
        "fres" | "fourwire" => {
            let resp = query_scpi(addr, ":MEAS:FRES?").await?;
            println!("{}", resp.trim_end());
        }
        "freq" | "frequency" => {
            let resp = query_scpi(addr, ":MEAS:FREQ?").await?;
            println!("{}", resp.trim_end());
        }
        "cont" | "continuity" => {
            let resp = query_scpi(addr, ":MEAS:CONT?").await?;
            println!("{}", resp.trim_end());
        }
        "diode" => {
            let resp = query_scpi(addr, ":MEAS:DIODe?").await?;
            println!("{}", resp.trim_end());
        }
        "cap" | "capacitance" => {
            let resp = query_scpi(addr, ":MEAS:CAP?").await?;
            println!("{}", resp.trim_end());
        }
        _ => {
            eprintln!("dmm <dcv|acv|dci|aci|res|fres|freq|cont|diode|cap>");
        }
    }
    Ok(true)
}
