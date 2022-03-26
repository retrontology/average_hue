use std::fs::File;
use std::io::prelude::*;
use hueclient::Bridge;
use text_io::read;
use std::path::Path;
use uuid::Uuid;

const UUIDFILENAME: &str = "bridge.uuid";

fn main() {
    let bridge = get_bridge(Path::new(UUIDFILENAME)); 
    println!("the username was {}", bridge.username);
}

fn get_bridge(uuid_path: &Path) -> Bridge{
    let mut username: String = String::new();
    let display = uuid_path.display();
    if Path::new(UUIDFILENAME).exists() {
        let mut file = match File::open(&uuid_path){
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        match file.read_to_string(&mut username) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => return Bridge::discover_required().with_user(username),
        };
        
    } else {
        username = Uuid::new_v4().to_string();
        let mut file = match File::create(&uuid_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        let bridge = Bridge::discover_required();
        println!("Found hue bridge at {}. Press Hue Bridge button first, then press Enter.", bridge.ip);
        let _confirm: String = read!("{}\n");
        let outbridge = bridge.register_user(&username).unwrap();
        match file.write_all(username.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
        return outbridge;
    }
}
