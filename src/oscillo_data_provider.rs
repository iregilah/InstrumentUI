// src/oscillo_data_provider.rs
use crate::instrument::Instrument;
use std::env;
use std::error::Error;
use std::io;

#[derive(Debug, Clone)]
pub struct Waveform {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub x_label: String,
    pub y_label: String,
    pub x_unit: String,
    pub y_unit: String,
}

#[derive(Debug, Clone, Copy)]
struct RigolPreamble {
    points: usize,
    x_inc: f64,
    x_origin: f64,
    x_ref: f64,
    y_inc: f64,
    y_origin: f64,
    y_ref: f64,
}

pub fn fetch_waveform_from_env(channel: u8) -> Result<Waveform, Box<dyn Error>> {
    let addr = env::var("INSTRUMENT_ADDR")?;
    fetch_rigol_ds1000z_waveform(&addr, channel)
}

pub fn fetch_rigol_ds1000z_waveform(addr: &str, channel: u8) -> Result<Waveform, Box<dyn Error>> {
    let mut instr = Instrument::connect(addr)?;

    // NORM mode: screen content (gyors, és azt tükrözi, ami a kijelzőn van)
    fetch_rigol_ds1000z_waveform_from_connected(&mut instr, channel)
}

fn fetch_rigol_ds1000z_waveform_from_connected(
    instr: &mut Instrument,
    channel: u8,
) -> Result<Waveform, Box<dyn Error>> {
    let chan = channel.clamp(1, 4);

    // Channel select
    instr.write(&format!(":WAV:SOUR CHAN{}", chan))?;

    // Best-effort configuration (ignore errors for compatibility)
    // A kijelzőn látható jelhez NORM a jó (DS1000Z: 1200 pont / teljes képernyőszélesség).
    let _ = instr.write(":WAV:MODE NORM");
    let _ = instr.write(":WAV:FORM BYTE");

    // Preamble (try a few variants)
    let preamble_str =
        query_line_fallback(instr, &[":WAV:PRE?", ":WAV:PREAMBLE?", ":WAV:PREamble?"])?;
    let pre = parse_rigol_preamble(&preamble_str)?;

    // Best-effort: try to request full record in one block
    let _ = instr.write(":WAV:STAR 1");
    if pre.points > 0 {
        let _ = instr.write(&format!(":WAV:STOP {}", pre.points));
    }

    // Data block
    instr.write(":WAV:DATA?")?;
    let block = instr.read_block()?;
    let payload = extract_ieee4882_payload(&block)?;

    let n = payload.len();
    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);

    for (i, &b) in payload.iter().enumerate() {
        let code = b as f64;
        // Rigol DS1000Z scaling:
        //   X = (i - XREF) * XINC + XORIG
        //   Y = (code - YREF) * YINC + YORIG
        let xv = (i as f64 - pre.x_ref) * pre.x_inc + pre.x_origin;
        let yv = (code - pre.y_ref) * pre.y_inc + pre.y_origin;
        x.push(xv);
        y.push(yv);
    }

    Ok(Waveform {
        x,
        y,
        x_label: "Time".to_owned(),
        y_label: format!("C{}", chan),
        x_unit: "s".to_owned(),
        y_unit: "V".to_owned(),
    })
}

fn query_line(instr: &mut Instrument, cmd: &str) -> io::Result<String> {
    instr.write(cmd)?;
    instr.read_line()
}

fn query_line_fallback(instr: &mut Instrument, cmds: &[&str]) -> Result<String, Box<dyn Error>> {
    let mut last_err: Option<io::Error> = None;
    for &cmd in cmds {
        match query_line(instr, cmd) {
            Ok(s) => return Ok(s),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err
        .unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "SCPI query failed"))
        .into())
}

fn parse_rigol_preamble(s: &str) -> Result<RigolPreamble, Box<dyn Error>> {
    // Expected CSV fields (Rigol DS1000Z):
    // FORMAT,TYPE,POINTS,COUNT,XINCR,XORIG,XREF,YINCR,YORIG,YREF
    let parts: Vec<&str> = s
        .trim()
        .split(',')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    if parts.len() < 10 {
        return Err(format!(
            "Rigol WAV preamble has too few fields ({}): '{}'",
            parts.len(),
            s.trim()
        )
        .into());
    }

    let points = parts[2].parse::<usize>().unwrap_or(0);
    let x_inc = parts[4].parse::<f64>()?;
    let x_origin = parts[5].parse::<f64>()?;
    let x_ref = parts[6].parse::<f64>()?;
    let y_inc = parts[7].parse::<f64>()?;
    let y_origin = parts[8].parse::<f64>()?;
    let y_ref = parts[9].parse::<f64>()?;

    Ok(RigolPreamble {
        points,
        x_inc,
        x_origin,
        x_ref,
        y_inc,
        y_origin,
        y_ref,
    })
}

fn extract_ieee4882_payload(buf: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if buf.is_empty() {
        return Ok(Vec::new());
    }

    // Some lower layers may already strip the IEEE488.2 header.
    if buf[0] != b'#' {
        return Ok(buf.to_vec());
    }

    if buf.len() < 2 {
        return Err("IEEE488.2 block too short".into());
    }

    let ndigits = (buf[1] as char)
        .to_digit(10)
        .ok_or("IEEE488.2 block: invalid digit count")? as usize;

    if ndigits == 0 {
        return Err("IEEE488.2 indefinite-length blocks are not supported".into());
    }

    let len_start = 2;
    let len_end = len_start + ndigits;
    if buf.len() < len_end {
        return Err("IEEE488.2 block: missing length digits".into());
    }

    let len_str = std::str::from_utf8(&buf[len_start..len_end])?;
    let data_len = len_str.parse::<usize>()?;

    let data_start = len_end;
    let data_end = data_start + data_len;
    if buf.len() < data_end {
        return Err("IEEE488.2 block: truncated payload".into());
    }

    Ok(buf[data_start..data_end].to_vec())
}
