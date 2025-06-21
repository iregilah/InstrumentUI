// src/io.rs

//! Fájl‑ és adattovábbítások (képernyőkép, hullámforma, CSV, setup,
//! valamint arbitrary‑waveform feltöltés).

use std::{error::Error, net::SocketAddr};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::utils::{read_ieee_block, write_file};
pub use crate::utils::parse_source_arg;
/// Képernyőkép letöltése PNG‑ben.
pub async fn fetch_screenshot(addr: &SocketAddr, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b":DISP:DATA?\n").await?;
    let data = read_ieee_block(&mut stream).await?;
    write_file(filename, &data).await?;
    println!("Screenshot saved → {}", filename);
    Ok(())
}

/// Hullámforma (BYTE formátum) letöltése – csatorna → fájl.
pub async fn fetch_waveform(
    addr: &SocketAddr,
    chan: &str,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr).await?;
    stream
        .write_all(format!(":WAV:SOUR {}\n", chan.to_ascii_uppercase()).as_bytes())
        .await?;
    stream.write_all(b":WAV:MODE NORM\n:WAV:FORM BYTE\n:WAV:DATA?\n").await?;
    let data = read_ieee_block(&mut stream).await?;
    write_file(filename, &data).await?;
    println!("Waveform saved → {}", filename);
    Ok(())
}

/// Teljes felbontású CSV‑export (RAW módban, chunkoltan).
pub async fn fetch_csv(
    addr: &SocketAddr,
    chan: &str,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    // --- lekérdezzük a skálázási paramétereket ---------------------------
    let mut dev = crate::lxi::Lxi::connect(addr).await?;
    dev.send(&format!(":WAV:SOUR {}", chan.to_ascii_uppercase())).await?;
    dev.send(":WAV:MODE RAW").await?;
    dev.send(":WAV:FORM BYTE").await?;
    dev.send(":WAV:STAR 1").await?;
    dev.send(":WAV:STOP 25000000").await?; // max‑range, a műszer úgyis visszavág
    let total_pts: usize = dev.query(":WAV:STOP?").await?.parse()?;
    let x_inc: f64 = dev.query(":WAV:XINC?").await?.parse()?;
    let x_org: f64 = dev.query(":WAV:XOR?").await?.parse()?;
    let y_inc: f64 = dev.query(":WAV:YINC?").await?.parse()?;
    let y_org: f64 = dev.query(":WAV:YOR?").await?.parse()?;
    let y_ref: f64 = dev.query(":WAV:YREF?").await?.parse()?;
    drop(dev);

    // --- fájl nyitása, fejléc --------------------------------------------
    let mut f = File::create(filename).await?;
    f.write_all(b"Time(s),Voltage(V)\n").await?;

    // --- adat letöltés chunkonként ----------------------------------------
    let chunk = 250_000;
    let mut tcp = TcpStream::connect(addr).await?;
    let mut start = 1usize;
    while start <= total_pts {
        let stop = (start + chunk - 1).min(total_pts);
        tcp.write_all(format!(":WAV:STAR {}\n:WAV:STOP {}\n:WAV:DATA?\n", start, stop).as_bytes())
            .await?;
        let payload = read_ieee_block(&mut tcp).await?;

        // CSV‑sorok előállítása
        let mut out = String::with_capacity(payload.len() * 20);
        for (i, b) in payload.iter().enumerate() {
            let idx = start - 1 + i;
            let t = x_org + (idx as f64) * x_inc;
            let v = ((*b as f64) - y_org - y_ref) * y_inc;
            out.push_str(&format!("{:.6e},{:.6e}\n", t, v));
        }
        f.write_all(out.as_bytes()).await?;

        start = stop + 1;
    }
    println!("CSV saved → {}", filename);
    Ok(())
}

/// Setup‑fájl lementése bináris blokkban.
pub async fn save_config(addr: &SocketAddr, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b":SYST:SETup?\n").await?;
    let blob = read_ieee_block(&mut stream).await?;
    write_file(filename, &blob).await?;
    println!("Instrument setup saved → {}", filename);
    Ok(())
}

/// Setup‑fájl visszatöltése a műszerbe.
pub async fn load_config(addr: &SocketAddr, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(filename).await?;
    let mut data = Vec::new();
    f.read_to_end(&mut data).await?;

    let len_str = data.len().to_string();
    if len_str.len() > 9 || data.is_empty() {
        return Err("Invalid or too large setup file".into());
    }
    let header = format!("#{}{}", len_str.len(), len_str);

    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b":SYST:SETup ").await?;
    stream.write_all(header.as_bytes()).await?;
    stream.write_all(&data).await?;
    stream.flush().await?;
    println!("Instrument setup loaded from {}", filename);
    Ok(())
}

/// Arbitrary‑waveform feltöltése (DAC pontok 0‑16383).
///
/// A bemeneti fájl lehet CSV vagy whitespace‑delimitált lista; ha az
/// értékek ‑1…+1 vagy 0…1 tartományban vannak, automatikus skálázás
/// történik.
pub async fn load_arb(
    addr: &SocketAddr,
    ch: u8,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    // --- fájl beolvasása --------------------------------------------------
    let mut f = File::open(filename).await?;
    let mut text = String::new();
    f.read_to_string(&mut text).await?;
    let tokens = text.replace(',', " ");

    // --- normalizálás / konvertálás 0‑16383 skálára -----------------------
    let mut vals_f = Vec::<f64>::new();
    for t in tokens.split_whitespace() {
        if let Ok(v) = t.parse::<f64>() {
            vals_f.push(v);
        }
    }
    if vals_f.is_empty() {
        return Err("No numeric data found in file".into());
    }

    let min = vals_f
        .iter()
        .copied()
        .fold(f64::INFINITY, |a, b| a.min(b));
    let max = vals_f
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));

    let to_raw = |v: f64| -> u16 {
        let mut r = v;
        if min >= 0.0 && max <= 1.0 {
            r = (v * 16383.0).round();
        } else if min >= -1.0 && max <= 1.0 {
            r = ((v + 1.0) * 8191.5).round();
        } else {
            // +/-8192 közé tolás
            if r < 0.0 {
                r += 8192.0;
            }
        }
        r.clamp(0.0, 16383.0) as u16
    };

    let data: Vec<u16> = vals_f.into_iter().map(to_raw).collect();

    // --- feltöltés a műszerbe --------------------------------------------
    let mut stream = TcpStream::connect(addr).await?;
    stream
        .write_all(format!(":TRAC{ch}:DATA:POIN volatile,{}\n", data.len()).as_bytes())
        .await?;

    let mut cmd = String::with_capacity(20 + data.len() * 6);
    cmd.push_str(&format!(":TRAC{ch}:DATA:DAC volatile,"));
    for (i, v) in data.iter().enumerate() {
        cmd.push_str(&v.to_string());
        if i + 1 != data.len() {
            cmd.push(',');
        }
    }
    cmd.push('\n');
    stream.write_all(cmd.as_bytes()).await?;
    stream.flush().await?;

    println!("Arb‑waveform ({}) uploaded to channel {}", data.len(), ch);
    Ok(())
}

