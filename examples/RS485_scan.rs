use rigol_cli::aggregator::Aggregator;
use serde_json::json;

fn main() {
    let mut aggr = Aggregator::new().expect("Aggregátor init hiba");
    // Feltételezzük, hogy a config.json engedélyezi a Serial/RS485 interfészt:
    // pl. "RS485": { "enabled": true } vagy "Serial": { "enabled": true }
    println!("Elérhető soros portok: {}", aggr.lsif());
    // Példa kimenet: "MODBUS-RTU/RS-485 on COM3; MODBUS-RTU/RS-485 on COM5"

    // 1. Válasszunk ki egy soros portot (pl. COM5) a felderítéshez:
    let port_name = "COM5";  // Ezt szükség szerint módosítsd a valós port nevére
    // 2. Port konfiguráció (9600 baud, 8N1, pl. RS-485 eszköz esetén):
    let settings = json!({
        "baud": 9600,
        "data_bits": 8,
        "parity": "N",
        "stop_bits": 1
    });
    if let Err(e) = aggr.configif(Some("Serial"), Some(port_name), Some(&settings)) {
        eprintln!("Hiba a port konfigurálásakor: {}", e);
        return;
    }
    println!("Port {} konfigurálva (9600 8N1)", port_name);

    // 3. Eszköz keresése a megadott porton (akár ID megadásával):
    // Ha ismerjük a slave címet vagy azonosítót, ide beírhatjuk.
    // Itt üres stringet adunk, hogy *IDN? alapon próbálja meg (egy eszközt feltételezve a buszon).
    if let Some(db) = aggr.connect("Serial", port_name, "") {
        // Sikerült kapcsolatba lépni egy eszközzel a soros porton
        for (&uuid, info) in db {
            if info.port == port_name {
                println!("Eszköz csatlakoztatva a {} portra: UUID {uuid}, {} {}",
                         port_name,
                         info.vendor.clone().unwrap_or_default(),
                         info.model.clone().unwrap_or_default());
            }
        }
        // 4. Küldjünk egy parancsot a csatlakozott eszköznek, pl. ismét *IDN?
        let uuids: Vec<u32> = aggr.connected_instruments.keys().cloned().collect();
        if let Some(&uuid) = uuids.first() {
            let response = aggr.send_to(&[uuid], "*IDN?");
            if let Ok(answer) = &response[0].1 {
                println!("Válasz az eszköztől: {}", answer);
            } else if let Err(err) = &response[0].1 {
                println!("Hiba történt a parancs küldésekor: {}", err);
            }
        }
    } else {
        println!("Nem található eszköz a {} porton.", port_name);
    }

    // 5. Port bontása és erőforrás felszabadítása
    aggr.disconnect_interface("Serial", Some(port_name));
    println!("{} port lecsatlakoztatva és lezárva.", port_name);
}
