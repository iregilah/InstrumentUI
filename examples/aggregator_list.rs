// examples/aggregator_list.rs
use rigol_cli::aggregator::Aggregator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut agg = Aggregator::new()?;
    let db = agg.discover_all();
    for (id, info) in db {
        println!(
            "#{id}: {:?} {:?} via {} on {}",
            info.vendor, info.model, info.interface, info.port
        );
    }
    Ok(())
}