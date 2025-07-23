


// src/main.rs
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    // Initialize Qt GUI application and QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        // Load the QML UI from resources
        engine.load(&QUrl::from("qrc:/main.qml"));
    }
    if let Some(app) = app.as_mut() {
        // Start the Qt event loop
        app.exec();
    }
}

/*// src/main.rs
use std::{env, net::SocketAddr};
use std::error::Error;

use rigol_cli::prelude::*;           // <- a mini‑prelude
use rigol_cli::{cli, repl};          // a két belső almodul

/// Ha nem kapunk IP‑címet a parancssorban, erre csatlakozunk.
const DEFAULT_ADDR: &str = "169.254.50.23:5555";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /* ---------- argumentum‑feldolgozás -------------------------------- */
    let args: Vec<String> = env::args().collect();

    // 0.: a bináris neve, 1.: (opcionálisan) IP[:PORT], 2…: parancsok
    let addr_str = match args.get(1) {
        // ha van IP‑paraméter → formázzuk :5555‑re, ha nincs port
        Some(s) if !s.starts_with('-') => {
            if s.contains(':') { s.clone() } else { format!("{s}:5555") }
        }
        // nincs paraméter → alapértelmezett IP
        _ => DEFAULT_ADDR.to_owned(),
    };
    let addr: SocketAddr = addr_str.parse()?;   // hibakezelés a parse‑nál

    /* ---------- ➊ „öröklött” minimál funkció -------------------------- */
    // minden induláskor biztosítjuk, hogy a CH1 látható legyen
    // (ha ez zavaró, egy --no-init flaggel feltételesen kikapcsolható,
    //  de most a kérés kizárólag az integrálás volt).
    send_scpi(&addr, ":CHAN1:DISP ON").await?;
    println!("(Init) CH1 display → ON  [{addr}]");

    /* ---------- ➋ CLI kontra REPL ------------------------------------- */
    if args.len() > 2 {
        // van extra argumentum → egylövéses parancssori mód
        cli::run_cli(&addr, &args[2..]).await?;
    } else {
        // csak IP (vagy az sem) → interaktív prompt
        repl::run_repl(&addr).await?;
    }

    Ok(())
}*/