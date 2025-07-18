use rigol_cli::aggregator::Aggregator;

fn main() {
    let mut aggr = Aggregator::new().expect("Aggregátor init hibás");

    println!("USB interfészek: {}", aggr.lsif());

    // --- USB‑eszközök felderítése ------------------------------
    aggr.discover_all();

    // ➊  Csak a UUID‑ket gyűjtjük ki
    let usb_uuids: Vec<u32> = aggr
        .connected_instruments
        .iter()
        .filter(|(_, info)| info.interface.starts_with("USB"))
        .map(|(&uuid, _)| uuid)
        .collect();

    // ➋  Kiírás – külön blokkban, hogy az immut‑kölcsönzés itt véget érjen
    {
        if usb_uuids.is_empty() {
            println!("Nem található USB‑s műszer.");
        } else {
            println!("USB‑n talált műszerek:");
            for uuid in &usb_uuids {
                let info = &aggr.connected_instruments[uuid];
                println!(
                    "  UUID {}: {} {} – {} (Azonosító: {})",
                    uuid,
                    info.vendor.clone().unwrap_or_default(),
                    info.model.clone().unwrap_or_default(),
                    info.instrument_type.clone().unwrap_or_else(|| "ismeretlen típus".to_string()),
                    info.identifier
                );
            }
        }
    } // <‑‑ az immutábilis kölcsönzés itt lejár

    // ➌  Most már szabadon kérdezhetünk mutábil módon
    for uuid in &usb_uuids {
        let result = aggr.send_to(&[*uuid], "*IDN?");
        let (_id, resp) = &result[0];
        match resp {
            Ok(answer) => println!("Eszköz {} válasza: {}", uuid, answer),
            Err(err)   => println!("USB eszköz {} kommunikációs hiba: {}", uuid, err),
        }
    }

    aggr.disconnect_all();
}