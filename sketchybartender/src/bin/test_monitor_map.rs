use sketchybartender::monitor_map::MonitorMapper;

fn main() {
    let mapper = MonitorMapper::new();
    let mappings = mapper.get_mappings();

    println!("Final monitor mappings:");
    for (sb_id, aero_id) in &mappings {
        println!("  Sketchybar display {} -> Aerospace monitor {}", sb_id, aero_id);
    }

    if mappings.is_empty() {
        println!("WARNING: No mappings found!");
    }
}
