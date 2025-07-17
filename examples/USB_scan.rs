use rigol_cli::aggregator::Aggregator;

fn main() {
    let mut aggr = Aggregator::new().expect("Aggregátor init hibás");
    // Feltételezzük, hogy a config.json engedélyezi az USB interfészt: `"USB": { "enabled": true }`
    println!("USB interfészek: {}", aggr.lsif());
    // Pl. "USB on Bus 001 Root Hub; USB on Bus 002 Root Hub"

    // USB eszközök felderítése
    aggr.discover_all();
    let mut usb_devices = Vec::new();
    for (&uuid, info) in &aggr.connected_instruments {
        if info.interface.starts_with("USB") {
            usb_devices.push((uuid, info));
        }
    }
    if usb_devices.is_empty() {
        println!("Nem található USB-s műszer.");
    } else {
        println!("USB-n talált műszerek:");
        for (uuid, info) in &usb_devices {
            println!("  UUID {uuid}: {} {} – {} (Azonosító: {})",
                     info.vendor.clone().unwrap_or_default(),
                     info.model.clone().unwrap_or_default(),
                     info.instrument_type.clone().unwrap_or("ismeretlen típus".to_string()),
                     info.identifier);
        }
    }

    // Próbáljunk meg parancsot küldeni az USB eszközöknek (*IDN?).
    for (uuid, _) in usb_devices {
        let result = aggr.send_to(&[uuid], "*IDN?");
        let (_id, resp) = &result[0];
        match resp {
            Ok(answer) => println!("Eszköz {uuid} válasza: {}", answer),
            Err(err) => println!("USB eszköz {uuid} kommunikációs hiba: {}", err),
        }
    }

    aggr.disconnect_all();
}
