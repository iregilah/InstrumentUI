use rigol_cli::aggregator::Aggregator;

fn main() {
    // 1. Aggregátor inicializálása (LXI engedélyezve a config alapján)
    let mut aggr = Aggregator::new().expect("Aggregátor inicializálás sikertelen");
    println!("Elérhető interfészek: {}", aggr.lsif());
    // Példa kimenet: "LXI/TCP/MODBUS-TCP on adapter enp3s0" stb.

    // 2. Eszközök felderítése minden interfészen (LXI esetén a config.json alapján)
    let devices = aggr.discover_all();
    if devices.is_empty() {
        println!("Nem található eszköz automatikus felderítéssel.");
    } else {
        println!("Felderített eszközök:");
        for (&uuid, info) in devices {
            println!("  UUID {uuid}: {} {} – {}",
                     info.vendor.clone().unwrap_or_default(),
                     info.model.clone().unwrap_or_default(),
                     info.instrument_type.clone().unwrap_or("Ismeretlen típus".to_string()));
        }
    }

    // 3. Ha nem találtunk semmit, megpróbálunk manuálisan csatlakozni egy ismert LXI eszközhöz.
    // (Adjuk meg az LXI interfész nevét, adapter portját és az eszköz IP-címét.)
    if devices.is_empty() {
        // Példa IP-cím – ezt módosítsd a valós eszközöd IP-címére:
        let manual_ip = "169.254.50.23";
        // Az adapter neve lehet pl. "adapter enp3s0" vagy "adapter Ethernet" – lsd. lsif() kimenet.
        let lxi_adapter = aggr.lsif()
            .split(';')
            .find(|s| s.contains("LXI"))
            .and_then(|entry| entry.split(" on ").last())
            .unwrap_or("");
        if lxi_adapter.is_empty() {
            eprintln!("Nincs LXI adapter a rendszerben.");
            return;
        }
        println!("\nManuális csatlakozás LXI eszközhöz: {} @ {}", manual_ip, lxi_adapter);
        if let Some(db) = aggr.connect("LXI", lxi_adapter, manual_ip) {
            // Sikeres csatlakozás, eszköz hozzáadva az adatbázishoz
            for (&uuid, info) in db {
                if info.identifier == manual_ip {
                    println!("Csatlakoztatva: UUID {uuid}, {} {}",
                             info.vendor.clone().unwrap_or_default(),
                             info.model.clone().unwrap_or_default());
                }
            }
        } else {
            eprintln!("Nem sikerült csatlakozni a LXI eszközhöz ({})", manual_ip);
            return;
        }
    }

    // 4. Küldjünk parancsot minden csatlakoztatott eszköznek (pl. *IDN? az azonosításhoz)
    let uuids: Vec<u32> = aggr.connected_instruments.keys().cloned().collect();
    for (uuid, result) in aggr.send_to(&uuids, "*IDN?") {
        match result {
            Ok(resp) => println!("Eszköz {uuid} válasza a *IDN?-re: {}", resp),
            Err(err) => println!("Eszköz {uuid} hiba a küldéskor: {}", err),
        }
    }

    // 5. Összes eszköz lekapcsolása
    aggr.disconnect_all();
    println!("Minden eszköz lecsatlakoztatva.");
}
