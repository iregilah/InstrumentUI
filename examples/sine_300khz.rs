//! 300 kHz‑es, 2 V‑pp szinusz generálása a beépített AWG‑n,
//! majd a szkóp idő‑/feszültség‑skáláinak illesztése úgy, hogy
//! nagyjából **két periódus** férjen el a teljes 10 div‑es ablakban.
//
//  Futás :
//      cargo run --example sine_300khz -- --ip 169.254.50.23:5555
//
//  Ha nincs --ip argumentum, a DEFAULT_ADDR‑t (lib) veszi fel.

/*
Mit kell látnod a képernyőn?
Terület	Elvárt megjelenés
AWG OUT 1	300 kHz szinusz, 2 V‑pp (±1 V). Előlapi OUTPUT 1 LED zöld.
Időskála	~0.5–1 µs/div tartomány. A teljes 10‑div szélességben ~2 teljes hullám látható.
Vertikális skála	0.5 V/div – így a jel ±1 V amplitúdója ±2 div‑re esik a középtől (4 div teljes magasság).
Trigger	SINGLE után stabil, nem „szalad” a jel.

Tipp
Ha a firmware 0.667 µs/div‑re nem áll be pontosan, használd manuálisan
a legközelebbi (0.5 vagy 1 µs/div) értéket, ekkor kb. 1.5‑3 periódus
jelenik meg – ez nem hiba, csak kerekítés a fix lépcsőhöz.

 */

//! 300 kHz‑es, 2 V‑pp szinusz kimenet + mérés a CH1‑en
//! Pontosan 2 periódus a 10 div vízszintes ablakban.
//
//  Futás :
//      cargo run --example sine_300khz_ch1 -- --ip 169.254.50.23:5555
//
//  Ha nincs --ip, a könyvtárban definiált DEFAULT_ADDR érvényesül.

use rigol_cli::lxi::send_scpi;
use std::{net::SocketAddr, time::Duration};
use tokio::time::sleep;

/* -------------------------- CLI‑paraméterek -------------------------- */

fn arg_after(flag: &str) -> Option<String> {
    let mut it = std::env::args().skip_while(|s| s != flag);
    it.next()?;           // a flag
    it.next()             // a flag utáni elem
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip   = arg_after("--ip"  ).unwrap_or_else(|| "169.254.50.23:5555".into());
    let freq = arg_after("--freq").unwrap_or_else(|| "300000".into()).parse::<f64>()?;
    let vpp  = arg_after("--vpp" ).unwrap_or_else(|| "2.0".into()).parse::<f64>()?;

    let addr: SocketAddr = ip.parse()?;
    println!("Kapcsolódás: {addr}");
    println!("AWG  : {freq} Hz  |  {vpp} Vpp");
    println!("Cél  : pontosan 2 periódus a 10 div‑en\n");

    /* -------------------- 1) AWG beállítás --------------------------- */
    // A Rigol AWG parancsa: :SOURn:APPL:SIN <FREQ>,<AMP>,<OFFSET>,<PHASE>
    send_scpi(&addr, &format!(":SOUR1:APPL:SIN {freq},{vpp},0,0")).await?;
    send_scpi(&addr, ":OUTPUT1 ON").await?;

    /* -------------------- 2) Szkóp CH1 skálák ------------------------- */

    // --- vertikális skála ------------------------------------------------
    // 8 vertikális osztás → érdemes ~4 osztást lefedni:
    //     scale = Vpp / 4  (így ±2 div‑et használ)
    let v_scale = vpp / 4.0;
    send_scpi(&addr, ":CHAN1:DISP ON").await?;
    send_scpi(&addr, &format!(":CHAN1:SCAL {v_scale}")).await?;
    send_scpi(&addr, ":CHAN1:OFFS 0").await?;

    // --- horizontális skála ---------------------------------------------
    // 2 periódus / 10 div  ⇒  (2 / f) / 10  = 0.2 * T
    let t_scale_exact = 2.0 / freq / 10.0;

    // A DS/MSO‑Z sorozat elfogad tetszőleges értéket, de ha szükséges, itt
    // kerekíthetnénk a 1‑2‑5 lépcsőhöz.  Most megpróbáljuk a pontos értéket:
    send_scpi(&addr, &format!(":TIM:SCAL {t_scale_exact}")).await?;

    /* -------------------- 3) Trigger beállítás ------------------------ */
    send_scpi(&addr, ":TRIG:MODE EDGE").await?;
    send_scpi(&addr, ":TRIG:EDGE:SOUR CHAN1").await?;
    send_scpi(&addr, ":TRIG:EDGE:SLOP POS").await?;
    send_scpi(&addr, ":TRIG:EDGE:LEV 0").await?;

    /* -------------------- 4) SINGLE futtatás -------------------------- */
    send_scpi(&addr, ":SINGLE").await?;
    sleep(Duration::from_millis(500)).await;

    /* -------------------- Jelentés ------------------------------------ */
    println!("Beállítás kész.");
    println!("Vertikális skála : {v_scale:.3} V/div  (eljárás = Vpp / 4)");
    println!("Időskála         : {t_scale_exact:.6e} s/div  (2 periódus / 10 div)");
    println!("Trigger          : CH1, pozitív él, 0 V");
    println!("\n✦ A kijelzőn 10 osztáson **pontosan két periódus** kell megjelenjen.");
    println!("✦ Ha a készülék a skálát kerekítené, ±1‑2 % eltérés még elfogadható.\n");

    Ok(())
}
