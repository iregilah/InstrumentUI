//! Parancs‑dispatcher: a CLI‑ből ide fut be minden egyes parancssor.

use std::{error::Error, net::SocketAddr};

pub mod basic;
pub mod trigger;
pub mod measure;
pub mod math;
pub mod display;
pub mod mask;
pub mod network;
pub mod logic;
pub mod awg;
pub mod acquire;
pub mod decode;

pub async fn dispatch(addr: &SocketAddr, cmd: &[String]) -> Result<(), Box<dyn Error>> {
    macro_rules! try_mod {
        ($m:ident) => {
            if $m::try_handle(addr, cmd).await? { return Ok(()); }
        };
    }

    try_mod!(basic);
    try_mod!(trigger);
    try_mod!(measure);
    try_mod!(math);
    try_mod!(display);
    try_mod!(mask);
    try_mod!(network);
    try_mod!(logic);
    try_mod!(awg);
    try_mod!(acquire);
    try_mod!(decode);

    eprintln!("Ismeretlen parancs");
    Ok(())
}