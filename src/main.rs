use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use hueclient::{Bridge, CommandLight};
use text_io::read;
use std::path::Path;
use uuid::Uuid;

const UUIDFILENAME: &str = "bridge.uuid";
const PERIOD: u16 = 1;
const COLOUR: u16 = 25500;
const OFF: bool = true;

fn main() {
    let bridge = get_bridge(Path::new(UUIDFILENAME));
    println!("the username was {}", bridge.username); 
    let lights = &bridge.get_all_lights().unwrap();
    let wait_time: u16 = PERIOD/2;
    let transition_time: u16 = PERIOD / 2 * 10;
    if OFF {
        let light_command = CommandLight::default().off();
        for light in lights {
            bridge.set_light_state(light.id, &light_command).unwrap();
        }
    } else {
        loop {
            let mut light_command = CommandLight {
                on: Some(true),
                bri: Some(254),
                hue: Some(COLOUR),
                sat: Some(254),
                ct: None,
                xy: None,
                transitiontime: Some(transition_time),
                alert: None,
                scene: None,
            };
            for light in lights {
                bridge.set_light_state(light.id, &light_command).unwrap();
            }
            thread::sleep(time::Duration::from_secs(wait_time.into()));
            light_command = CommandLight {
                on: Some(true),
                bri: Some(0),
                hue: Some(COLOUR),
                sat: Some(254),
                ct: None,
                xy: None,
                transitiontime: Some(transition_time),
                alert: None,
                scene: None,
            };
            for light in lights {
                bridge.set_light_state(light.id, &light_command).unwrap();
            }
            thread::sleep(time::Duration::from_secs(wait_time.into()));
        }
    }
    
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
        match file.write_all(outbridge.username.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
        return outbridge;
    }
}
