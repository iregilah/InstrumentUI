// src/commands/math.rs

//! MATH‑/FFT‑funkciók (egyszerű MATH‑operátorok + spektrumanalízis)

use std::{error::Error, net::SocketAddr};

use crate::{
    lxi::{send_scpi},
};
use crate::io::parse_source_arg;

/// Feldolgoz – ha a parancs ide tartozik, végrehajtja és `true`‑val tér vissza.
pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(false);
    }

    /* ------------------------------- FFT ------------------------------ */
    if cmd[0] == "fft" {
        if cmd.len() == 2 && cmd[1].eq_ignore_ascii_case("off") {
            send_scpi(addr, ":MATH:DISP OFF").await?;
        } else if cmd.len() >= 2 {
            let src = parse_source_arg(&cmd[1])?;
            send_scpi(addr, ":MATH:DISP ON").await?;
            send_scpi(addr, ":MATH:OPER FFT").await?;
            send_scpi(addr, &format!(":MATH:FFT:SOUR {}", src)).await?;
            if let Some(w) = cmd.get(2) {
                let win = match w.to_ascii_lowercase().as_str() {
                    "rect" | "rectangular" => "RECT",
                    "hann" | "hanning"     => "HANN",
                    "hamm" | "hamming"     => "HAMM",
                    "black" | "blackman"   => "BLACK",
                    _                      => "HANN",
                };
                send_scpi(addr, &format!(":MATH:FFT:WIND {}", win)).await?;
            }
        } else {
            eprintln!("fft <source>|off [window]");
        }
        return Ok(true);
    }

    /* --------------------------- MATH‑operátorok ---------------------- */
    if cmd[0] == "math" && cmd.len() >= 2 {
        if cmd[1].eq_ignore_ascii_case("off") {
            send_scpi(addr, ":MATH:DISP OFF").await?;
            return Ok(true);
        }

        let op = match cmd[1].to_ascii_lowercase().as_str() {
            "add" | "plus"           => "ADD",
            "sub" | "minus"          => "SUBT",
            "mul" | "times"          => "MULT",
            "div" | "divide"         => "DIV",
            "intg" | "integrate"     => "INTG",
            "diff" | "deriv"         => "DIFF",
            "sqrt"                   => "SQRT",
            "log"                    => "LOG",
            "ln"                     => "LN",
            "exp"                    => "EXP",
            "abs"                    => "ABS",
            "and"                    => "AND",
            "or"                     => "OR",
            "xor"                    => "XOR",
            "not"                    => "NOT",
            _ => {
                eprintln!("Ismeretlen math‑operátor");
                return Ok(true);
            }
        };

        let unary = matches!(op, "INTG" | "DIFF" | "SQRT" | "LOG" | "LN" | "EXP" | "ABS" | "NOT");
        if (unary && cmd.len() != 3) || (!unary && cmd.len() != 4) {
            eprintln!("math {} <src1>{}", op, if unary { "" } else { " <src2>" });
            return Ok(true);
        }

        let src1 = parse_source_arg(&cmd[2])?;
        let src2 = if unary { src1.clone() } else { parse_source_arg(&cmd[3])? };

        send_scpi(addr, ":MATH:DISP ON").await?;
        send_scpi(addr, &format!(":MATH:OPER {}", op)).await?;
        send_scpi(addr, &format!(":MATH:SOUR1 {}", src1)).await?;
        if !unary {
            send_scpi(addr, &format!(":MATH:SOUR2 {}", src2)).await?;
        }
        return Ok(true);
    }

    Ok(false)
}
