use rigol_cli::aggregator::Aggregator;

fn main() {
    let mut aggr = Aggregator::new().expect("Aggregátor init hiba");
    // Feltételezzük, hogy a config.json engedélyezi a GPIB interfészt: `"GPIB": { "enabled": true }`
    let ports = aggr.lsif();
    println!("GPIB interfészek: {}", ports);
    // Pl. "GPIB on NI GPIB-USB-HS, Bus 001 Device 004"

    // Ha van elérhető GPIB adapter:
    if ports.contains("GPIB") {
        // Kíséreljünk meg csatlakozni egy eszközhöz GPIB címmel (pl. címe 5)
        // A port string pontosan a lsif() által adott adapter neve legyen a csatlakozáshoz.
        let adapter_port = ports.split(';')
            .find(|s| s.contains("GPIB"))
            .and_then(|entry| entry.split(" on ").last())
            .unwrap_or("");
        let gpib_address = "5";  // Példa GPIB eszközcím
        println!("Megpróbálunk csatlakozni a GPIB eszközhöz címen {}...", gpib_address);
        if let Some(db) = aggr.connect("GPIB", adapter_port, gpib_address) {
            if db.values().any(|info| info.interface.starts_with("GPIB")) {
                println!("GPIB eszköz (cím {gpib_address}) hozzáadva az adatbázishoz.");
            }
        } else {
            println!("Nem található GPIB eszköz a(z) {} címen.", gpib_address);
        }
        // Próbáljunk parancsot küldeni, bár jelenleg nem implementált
        for (&uuid, info) in &aggr.connected_instruments {
            if info.interface.starts_with("GPIB") {
                let result = aggr.send_to(&[uuid], "*IDN?");
                match &result[0].1 {
                    Ok(resp) => println!("GPIB eszköz válasza: {}", resp),
                    Err(err) => println!("GPIB kommunikációs hiba (várható, nincs implementálva): {}", err),
                }
            }
        }
    } else {
        println!("Nincs GPIB adapter csatlakoztatva.");
    }

    aggr.disconnect_all();
}
