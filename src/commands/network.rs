// src/commands/network.rs

//! LAN konfiguráció: DHCP, statikus IP, MAC‑cím, stb.

use std::{error::Error, net::SocketAddr};

use crate::lxi::{query_scpi, send_scpi};

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() || (cmd[0] != "lan" && cmd[0] != "network") {
        return Ok(false);
    }
    if cmd.len() == 1 {
        eprintln!("lan <ip|mask|gw|dns|mac|dhcp|autoip|status|apply|static …>");
        return Ok(true);
    }

    match cmd[1].to_ascii_lowercase().as_str() {
        "ip" => simple(addr, ":LAN:IPAD", &cmd[2..]).await?,
        "mask" | "netmask" => simple(addr, ":LAN:SMAS", &cmd[2..]).await?,
        "gw" | "gateway"   => simple(addr, ":LAN:GAT",  &cmd[2..]).await?,
        "dns"              => simple(addr, ":LAN:DNS",  &cmd[2..]).await?,
        "mac" => {
            println!("{}", query_scpi(addr, ":LAN:MAC?").await?.trim_end());
        }
        "status" => {
            println!("{}", query_scpi(addr, ":LAN:STAT?").await?.trim_end());
        }
        "dhcp" => {
            if cmd.len() == 2 {
                println!("{}", query_scpi(addr, ":LAN:DHCP?").await?.trim_end());
            } else {
                let on = yes(&cmd[2]);
                send_scpi(addr, &format!(":LAN:DHCP {}", if on { "ON" } else { "OFF" })).await?;
            }
        }
        "autoip" | "auto" => {
            if cmd.len() == 2 {
                println!("{}", query_scpi(addr, ":LAN:AUT?").await?.trim_end());
            } else {
                let on = yes(&cmd[2]);
                send_scpi(addr, &format!(":LAN:AUT {}", if on { "ON" } else { "OFF" })).await?;
            }
        }
        "apply" => send_scpi(addr, ":LAN:APPL").await?,
        "init"  => send_scpi(addr, ":LAN:INIT").await?,
        "static" => {
            if cmd.len() < 5 {
                eprintln!("lan static <ip> <mask> <gw> [dns]");
            } else {
                send_scpi(addr, ":LAN:DHCP OFF").await?;
                send_scpi(addr, ":LAN:AUT OFF").await?;
                send_scpi(addr, ":LAN:MAN ON").await?;
                send_scpi(addr, &format!(":LAN:IPAD {}", cmd[2])).await?;
                send_scpi(addr, &format!(":LAN:SMAS {}", cmd[3])).await?;
                send_scpi(addr, &format!(":LAN:GAT {}",  cmd[4])).await?;
                if let Some(d) = cmd.get(5) {
                    send_scpi(addr, &format!(":LAN:DNS {}", d)).await?;
                }
                send_scpi(addr, ":LAN:APPL").await?;
            }
        }
        _ => eprintln!("Ismeretlen lan alcímke"),
    }
    Ok(true)
}

async fn simple(
    addr: &SocketAddr,
    scpi: &str,
    trailing: &[String],
) -> Result<(), Box<dyn Error>> {
    if trailing.is_empty() {
        println!("{}", query_scpi(addr, &format!("{scpi}?")).await?.trim_end());
    } else {
        send_scpi(addr, &format!("{scpi} {}", trailing[0])).await?;
    }
    Ok(())
}

fn yes(txt: &str) -> bool {
    matches!(txt.to_ascii_lowercase().as_str(), "on" | "1" | "yes" | "true")
}
