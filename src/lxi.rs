// src/lxi.rs

//! Vékony burkoló a `tokio_lxi` crate fölött.
//!
//! A fő cél, hogy kényelmes, egysoros `send()` / `query()` hívásokat
//! és egy minimalista RAII‑alapú kapcsolatkezelést kapjunk.

use std::{error::Error, net::SocketAddr};

use tokio_lxi::LxiDevice;

/// Egyszerű wrapper, amely a kapcsolat bontásakor automatikusan
/// lezárja a TCP‑csatornát.
pub struct Lxi {
    dev: LxiDevice,
}

impl Lxi {
    /// Csatlakozás az adott címhez (`IP:port`, tipikusan *5025*).
    pub async fn connect(addr: &SocketAddr) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            dev: LxiDevice::connect(addr).await?,
        })
    }

    /// SCPI‑utasítás küldése (nincs válasz).
    pub async fn send(&mut self, scpi: &str) -> Result<(), Box<dyn Error>> {
        self.dev.send(scpi).await?;
        Ok(())
    }

    /// SCPI‑lekérdezés: küld, majd várja a választ, és trimmeli a
    /// sorvégi `\n`‑t.
    pub async fn query(&mut self, scpi: &str) -> Result<String, Box<dyn Error>> {
        self.dev.send(scpi).await?;
        Ok(self.dev.receive().await?.trim_end().to_owned())
    }
}

/// Egylövéses küldés – kapcsolat létrehozása, parancs küldése,
/// majd azonnali bontás.
pub async fn send_scpi(addr: &SocketAddr, scpi: &str) -> Result<(), Box<dyn Error>> {
    let mut dev = Lxi::connect(addr).await?;
    dev.send(scpi).await
}

/// Egylövéses lekérdezés – kapcsolat létrehozása, parancs küldése,
/// válasz beolvasása, bontás.
pub async fn query_scpi(addr: &SocketAddr, scpi: &str) -> Result<String, Box<dyn Error>> {
    let mut dev = Lxi::connect(addr).await?;
    dev.query(scpi).await
}
