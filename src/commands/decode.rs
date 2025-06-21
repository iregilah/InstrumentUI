// src/commands/decode.rs

//! Soros‑busz dekóder be‑/kikapcsolás és konfiguráció (I²C / SPI / UART)

use std::{error::Error, net::SocketAddr};

use crate::{
    io::parse_source_arg,
    lxi::{send_scpi},
};

pub async fn try_handle(
    addr: &SocketAddr,
    cmd: &[String],
) -> Result<bool, Box<dyn Error>> {
    if cmd.is_empty() || cmd[0] != "decode" {
        return Ok(false);
    }
    if cmd.len() < 3 {
        eprintln!("decode <i2c|spi|uart>[1|2]  …  (részletek: lásd README)");
        return Ok(true);
    }

    /* ------------------------------------------------ I²C ----------- */
    if cmd[1].starts_with("i2c") {
        let idx = cmd[1]
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(1) as u8;
        if cmd.len() < 4 {
            eprintln!("decode i2c{} <scl_src> <sda_src> [7bit|rw] [thresh_V]", idx);
            return Ok(true);
        }
        let scl = parse_source_arg(&cmd[2])?;
        let sda = parse_source_arg(&cmd[3])?;
        send_scpi(addr, &format!(":DEC{idx}:MODE IIC")).await?;
        send_scpi(addr, &format!(":DEC{idx}:DISP ON")).await?;
        send_scpi(addr, &format!(":DEC{idx}:IIC:CLK {}", scl)).await?;
        send_scpi(addr, &format!(":DEC{idx}:IIC:DATA {}", sda)).await?;

        if cmd.len() >= 5 {
            match cmd[4].to_ascii_lowercase().as_str() {
                "7bit" | "7"     => send_scpi(addr, &format!(":DEC{idx}:IIC:ADDR 7BIT")).await?,
                "rw" | "readwrite" => send_scpi(addr, &format!(":DEC{idx}:IIC:ADDR RW")).await?,
                _ => {}
            }
        }
        if cmd.len() >= 6 {
            if let Ok(th) = cmd[5].parse::<f64>() {
                for ch in &[scl.as_str(), sda.as_str()] {
                    if let Some(n) = ch.strip_prefix("CHAN").and_then(|s| s.parse::<u8>().ok()) {
                        send_scpi(addr, &format!(":DEC{idx}:THRE:CHAN{n} {}", th)).await?;
                    }
                }
            }
        }
        return Ok(true);
    }

    /* ------------------------------------------------ SPI ----------- */
    if cmd[1].starts_with("spi") {
        let idx = cmd[1]
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(1) as u8;
        if cmd.len() < 6 {
            eprintln!("decode spi{} <clk> <mosi> <miso> <cs> [mode0‑3] [thresh_V]", idx);
            return Ok(true);
        }
        let clk  = parse_source_arg(&cmd[2])?;
        let mosi = parse_source_arg(&cmd[3])?;
        let miso = parse_source_arg(&cmd[4])?;
        let cs   = parse_source_arg(&cmd[5])?;

        send_scpi(addr, &format!(":DEC{idx}:MODE SPI")).await?;
        send_scpi(addr, &format!(":DEC{idx}:DISP ON")).await?;
        send_scpi(addr, &format!(":DEC{idx}:SPI:CLK {}",  clk)).await?;
        send_scpi(addr, &format!(":DEC{idx}:SPI:MOSI {}", mosi)).await?;
        send_scpi(addr, &format!(":DEC{idx}:SPI:MISO {}", miso)).await?;
        send_scpi(addr, &format!(":DEC{idx}:SPI:CS {}",   cs)).await?;

        /* üzemmód 0‑3 (CLK‑pola + mintavételi él) */
        if cmd.len() >= 7 {
            match cmd[6].chars().last() {
                Some('0') => { send_scpi(addr, &format!(":DEC{idx}:SPI:POL POS")).await?;
                    send_scpi(addr, &format!(":DEC{idx}:SPI:EDGE RISE")).await?; }
                Some('1') => { send_scpi(addr, &format!(":DEC{idx}:SPI:POL POS")).await?;
                    send_scpi(addr, &format!(":DEC{idx}:SPI:EDGE FALL")).await?; }
                Some('2') => { send_scpi(addr, &format!(":DEC{idx}:SPI:POL NEG")).await?;
                    send_scpi(addr, &format!(":DEC{idx}:SPI:EDGE FALL")).await?; }
                Some('3') => { send_scpi(addr, &format!(":DEC{idx}:SPI:POL NEG")).await?;
                    send_scpi(addr, &format!(":DEC{idx}:SPI:EDGE RISE")).await?; }
                _ => {}
            }
        }
        /* közös küszöbszint beállítása (ha adott) */
        if let Some(th) = cmd.get(7).and_then(|s| s.parse::<f64>().ok()) {
            for ch in &[clk.as_str(), mosi.as_str(), miso.as_str(), cs.as_str()] {
                if let Some(n) = ch.strip_prefix("CHAN").and_then(|s| s.parse::<u8>().ok()) {
                    send_scpi(addr, &format!(":DEC{idx}:THRE:CHAN{n} {}", th)).await?;
                }
            }
        }
        return Ok(true);
    }

    /* ------------------------------------------------ UART ---------- */
    if cmd[1].starts_with("uart") {
        let idx = cmd[1]
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(1) as u8;
        if cmd.len() < 4 {
            eprintln!("decode uart{} <rx_src|off> [tx_src|off] <baud> [bits] [parity] [stop] [thresh_V]", idx);
            return Ok(true);
        }

        /* -- RX/TX források és báud ---------------------------------- */
        let mut rx  = parse_source_arg(&cmd[2])?;
        let mut tx  = "OFF".to_owned();
        let mut pos = 3;
        if cmd.get(3).map_or(false, |s| s.parse::<f64>().is_err()) {
            tx  = parse_source_arg(&cmd[3])?;
            pos = 4;
        }
        let baud: f64 = cmd.get(pos).ok_or("baud?")?.parse()?;
        pos += 1;

        /* -- opcionális paraméterek ---------------------------------- */
        let mut bits   = 8u8;
        let mut parity = "NONE";
        let mut stop   = 1.0f64;
        let mut thr    = None;

        if let Some(s) = cmd.get(pos) {
            if let Ok(b) = s.parse::<u8>() { bits = b; pos += 1; }
        }
        if let Some(s) = cmd.get(pos) {
            match s.to_ascii_lowercase().as_str() {
                "odd"  => { parity = "ODD"; pos += 1; }
                "even" => { parity = "EVEN"; pos += 1; }
                "mark" => { parity = "MARK"; pos += 1; }
                "space"=> { parity = "SPACE"; pos += 1; }
                "none" | "off" => { parity = "NONE"; pos += 1; }
                _ => {}
            }
        }
        if let Some(s) = cmd.get(pos) {
            if let Ok(st) = s.parse::<f64>() { stop = st; pos += 1; }
        }
        if let Some(s) = cmd.get(pos) {
            thr = s.parse::<f64>().ok();
        }

        send_scpi(addr, &format!(":DEC{idx}:MODE UART")).await?;
        send_scpi(addr, &format!(":DEC{idx}:DISP ON")).await?;
        send_scpi(addr, &format!(":DEC{idx}:UART:RX {}", rx)).await?;
        send_scpi(addr, &format!(":DEC{idx}:UART:TX {}", tx)).await?;
        send_scpi(addr, &format!(":DEC{idx}:UART:BAUD {}", baud)).await?;
        send_scpi(addr, &format!(":DEC{idx}:UART:WIDT {}", bits)).await?;
        send_scpi(addr, &format!(":DEC{idx}:UART:PAR {}",  parity)).await?;
        send_scpi(addr, &format!(":DEC{idx}:UART:STOP {}", stop)).await?;

        if let Some(t) = thr {
            for ch in &[rx.as_str(), tx.as_str()] {
                if let Some(n) = ch.strip_prefix("CHAN").and_then(|s| s.parse::<u8>().ok()) {
                    send_scpi(addr, &format!(":DEC{idx}:THRE:CHAN{n} {}", t)).await?;
                }
            }
        }
        return Ok(true);
    }

    eprintln!("Ismeretlen decode‑típus");
    Ok(true)
}
