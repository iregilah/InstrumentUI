// src/utils.rs

//! Közös, más modulokban is gyakran használt segédfüggvények.

use std::{error::Error, path::Path};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

/// Csatorna‑, forrás‑, illetve jelazonosító argumentum elemzése.
/// A Rigol SCPI‑szintaxisához igazodik (CHAN1‑4, MATH, EXT, D0‑15 …).
pub fn parse_source_arg(arg: &str) -> Result<String, Box<dyn Error>> {
    let lower = arg.to_ascii_lowercase();

    // --- numerikus (1‑4)  → CHANnelN -------------------------------------
    if let Ok(ch) = arg.parse::<u8>() {
        if (1..=4).contains(&ch) {
            return Ok(format!("CHANnel{}", ch));
        }
    }

    // --- "chan…" prefix ---------------------------------------------------
    if lower.starts_with("chan") {
        let digits: String = arg.chars().filter(|c| c.is_ascii_digit()).collect();
        if let Ok(ch) = digits.parse::<u8>() {
            if (1..=4).contains(&ch) {
                return Ok(format!("CHANnel{}", ch));
            }
        }
    }

    // --- egyéb kulcsszavak -----------------------------------------------
    match lower.as_str() {
        "math"                     => return Ok("MATH".into()),
        "ext" | "external"         => return Ok("EXT".into()),
        "ext5"                     => return Ok("EXT5".into()),
        "line" | "acline"          => return Ok("ACLINE".into()),
        _                          => {}
    }

    // --- digitális csatornák (D0‑D15) ------------------------------------
    if lower.starts_with('d') {
        let digits: String = arg.chars().filter(|c| c.is_ascii_digit()).collect();
        if let Ok(idx) = digits.parse::<u8>() {
            if idx <= 15 {
                return Ok(format!("D{}", idx));
            }
        }
    }

    Err("Invalid source identifier".into())
}

/// IEEE‑488.2 bináris blokk beolvasása egy élő TCP‑streamről.
/// **Megjegyzés:**  a függvény a blokksorozat végén *nem* olvas további
/// sorvégi `\n`‑t – a hívó felelőssége, hogy szükség esetén kezelje.
pub async fn read_ieee_block(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
    // `#<Ndigit><len…><payload>`
    let mut hdr = [0u8; 2];
    stream.read_exact(&mut hdr).await?;
    if hdr[0] != b'#' {
        return Err("Binary block header '#' missing".into());
    }
    let ndigit = (hdr[1] - b'0') as usize;
    if ndigit == 0 || ndigit > 9 {
        return Err("Invalid Ndigit in binary block".into());
    }

    let mut len_buf = vec![0u8; ndigit];
    stream.read_exact(&mut len_buf).await?;
    let byte_count: usize = String::from_utf8(len_buf)?.parse()?;

    let mut data = vec![0u8; byte_count];
    stream.read_exact(&mut data).await?;
    Ok(data)
}

/// Aszinkron fájl‑kiírás (felülír).
pub async fn write_file<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut f = File::create(path).await?;
    f.write_all(data).await?;
    Ok(())
}
