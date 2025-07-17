use rigol_cli::aggregator::Aggregator;
use serde_json::json;

fn main() {
    let mut aggr = Aggregator::new().expect("Aggregátor init hiba");

    // 1. LXI eszköz csatlakoztatása (manuális, ha nincs automatikus felderítés)
    let lxi_ip = "169.254.50.23"; // példa IP – állítsd a saját műszered IP-jére
    let lxi_adapter = aggr.lsif()
        .split(';')
        .find(|s| s.contains("LXI"))
        .and_then(|entry| entry.split(" on ").last())
        .unwrap_or("");
    if !lxi_adapter.is_empty() {
        aggr.connect("LXI", lxi_adapter, lxi_ip);
    }

    // 2. Serial eszköz csatlakoztatása (manuális beállítás és kapcsolódás)
    let serial_port = "COM5"; // módosítsd a megfelelő portra
    let serial_settings = json!({ "baud": 9600, "data_bits": 8, "parity": "N", "stop_bits": 1 });
    aggr.configif(Some("Serial"), Some(serial_port), Some(&serial_settings)).ok();
    aggr.connect("Serial", serial_port, "");

    // 3. Minden csatlakoztatott eszköz listázása
    println!("Csatlakoztatott eszközök összesen: {}", aggr.connected_instruments.len());
    for (&uuid, info) in &aggr.connected_instruments {
        println!("  UUID {uuid}: {} {} @ {} ({})",
                 info.vendor.clone().unwrap_or_default(),
                 info.model.clone().unwrap_or_default(),
                 info.identifier,
                 info.interface);
    }

    // 4. Broadcast üzenet küldése minden eszköznek
    println!("\nBroadcast: *IDN? küldése minden eszköznek...");
    for (uuid, result) in aggr.broadcast("*IDN?") {
        match result {
            Ok(resp) => println!("[{uuid}] válasz: {}", resp.trim_end()),
            Err(err) => println!("[{uuid}] hiba: {}", err),
        }
    }

    aggr.disconnect_all();
}
