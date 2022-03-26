use std::fs::File;
use hueclient::Bridge;
use text_io::read;
use std::path::Path;
use uuid::Uuid;

const UUIDFILENAME: &str = "bridge.uuid";

fn main() {
    let username = Uuid::new_v4().to_string();
    println!("Generated UUID: {}", username);
    let bridge = Bridge::discover_required();
    println!("Found hue bridge at {}. Press Hue Bridge button first, then press Enter.", bridge.ip);
    let confirm: String = read!("{}\n");
    let hue_pass = bridge.register_user(&username).unwrap();
    //println!("the username was {}", bridge.username);
}

fn uuid_exists(uuid_path: &Path) -> bool {
    return false;
}

fn read_uuid(uuid_path: &Path){
    
}
