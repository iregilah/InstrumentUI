//! Digitális csatornák (LA) gyors­vezérlése

use std::{error::Error, net::SocketAddr};

use crate::lxi::send_scpi;

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }

    /* ---------------- rapid helpers ---------------- */
    match cmd[0].as_str() {
        "logic" if cmd.len() == 2 => {
            let on = yes(&cmd[1]);
            send_scpi(addr, &format!(":LA:STAT {}", if on { "ON" } else { "OFF" })).await?;
            if on {
                for d in 0..16 {
                    send_scpi(addr, &format!(":LA:DIG{d}:DISP ON")).await?;
                }
            }
            return Ok(true);
        }
        "digital" if cmd.len() == 3 => {
            if cmd[1].eq_ignore_ascii_case("all") {
                for d in 0..16 {
                    send_scpi(addr, &format!(":LA:DIG{d}:DISP {}", onoff(&cmd[2]))).await?;
                }
            } else if let Some(n) = cmd[1]
                .trim_start_matches('D')
                .parse::<u8>()
                .ok()
                .filter(|v| *v < 16)
            {
                send_scpi(addr, &format!(":LA:DIG{n}:DISP {}", onoff(&cmd[2]))).await?;
            } else {
                eprintln!("digital <D0‑D15|all> <on|off>");
            }
            return Ok(true);
        }
        "logicth" if cmd.len() == 3 => {
            let pod: u8 = cmd[1].parse().unwrap_or(0);
            if (1..=2).contains(&pod) {
                let v: f64 = cmd[2].parse()?;
                send_scpi(addr, &format!(":LA:POD{pod}:THR {}", v)).await?;
            } else {
                eprintln!("Pod index must be 1 or 2");
            }
            return Ok(true);
        }
        _ => {}
    }

    Ok(false)
}

/* -------------------------------------------------------------------- */

fn yes(s: &str) -> bool {
    matches!(s.to_ascii_lowercase().as_str(), "on" | "1" | "true" | "enable")
}
fn onoff(s: &str) -> &'static str { if yes(s) { "ON" } else { "OFF" } }
