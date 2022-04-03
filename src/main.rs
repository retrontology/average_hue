use hueclient::{Bridge, CommandLight, IdentifiedGroup, IdentifiedLight};
use kmeans_colors::{get_kmeans, Kmeans, Sort};
use palette::{FromColor, Hsv, IntoColor, Lab, Pixel, Srgb};
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{env, thread, time};
use text_io::read;
use uuid::Uuid;

const UUIDFILENAME: &str = "bridge.uuid";
const PERIOD: u16 = 3;
const COLOUR: u16 = 25500;
const OFF: bool = false;
const RUNS: u8 = 1;
const MAX_ITER: usize = 10;
const CONVERGE: f32 = 1.0;
const KMEANS_VERBOSE: bool = true;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bridge: Bridge = get_bridge(Path::new(UUIDFILENAME));
    println!("the username was {}", bridge.username);
    if OFF {
        let lights: Vec<IdentifiedLight> = bridge.get_all_lights().unwrap();
        let light_command = CommandLight::default().off();
        for light in lights {
            bridge.set_light_state(light.id, &light_command).unwrap();
        }
    } else {
        set_group_to_image(&bridge, &args[1], &args[2]);
    }
}

fn set_group_to_image(bridge: &Bridge, group_name: &str, img_path: &str) {
    let group: IdentifiedGroup = get_group(&bridge, &group_name).unwrap();
    let lights: Vec<String> = group.group.lights;
    let light_count: usize = lights.len();
    let img = image::open(&Path::new(img_path)).unwrap();
    let lab: Vec<Lab> = Srgb::from_raw_slice(&img.into_rgb8())
        .iter()
        .map(|x| x.into_format().into_color())
        .collect();
    let mut result = Kmeans::new();
    let mut rng = thread_rng();
    let seed: u64 = rng.gen_range(0..u64::MAX);
    for i in 0..RUNS {
        let run_result = get_kmeans(
            light_count,
            MAX_ITER,
            CONVERGE,
            KMEANS_VERBOSE,
            &lab,
            seed + i as u64,
        );
        if run_result.score < result.score {
            result = run_result;
        }
    }
    let mut res = Lab::sort_indexed_colors(&result.centroids, &result.indices);
    res.sort_unstable_by(|a, b| (b.percentage).partial_cmp(&a.percentage).unwrap());
    let colour_count = res.len();
    for i in 0..light_count {
        let colour_index = i % colour_count;
        let command = to_command_light(res[colour_index].centroid, None);
        let _result = bridge.set_light_state(lights[i].parse::<usize>().unwrap(), &command);
    }
}

fn to_command_light(colour: Lab, transition_time: Option<u16>) -> CommandLight {
    let hsv: Hsv = Hsv::from_color(colour);
    let brightness: u8 = (hsv.value * 253.0) as u8 + 1;
    let hue: u16 = (hsv.hue.to_positive_degrees() * 65535.0 / 360.0) as u16;
    let saturation: u8 = (hsv.saturation * 254.0) as u8;
    println!("B:{}\tH:{}:\tS{}", brightness, hue, saturation);
    return CommandLight {
        on: Some(true),
        bri: Some(brightness),
        hue: Some(hue),
        sat: Some(saturation),
        ct: None,
        xy: None,
        transitiontime: Some(transition_time.unwrap_or(20)),
        alert: None,
        scene: None,
    };
}

fn get_group(bridge: &Bridge, group_name: &str) -> Result<IdentifiedGroup, &'static str> {
    for group in bridge.get_all_groups().unwrap() {
        if group.group.name.eq(group_name) {
            return Ok(group);
        }
    }
    return Err("Group not found!");
}

fn pulse(bridge: &Bridge) {
    let lights = &bridge.get_all_lights().unwrap();
    let wait_time: u16 = PERIOD / 2;
    let transition_time: u16 = PERIOD / 2 * 10;
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

fn get_bridge(uuid_path: &Path) -> Bridge {
    let mut username: String = String::new();
    let display = uuid_path.display();
    if Path::new(UUIDFILENAME).exists() {
        let mut file = match File::open(&uuid_path) {
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
        println!(
            "Found hue bridge at {}. Press Hue Bridge button first, then press Enter.",
            bridge.ip
        );
        let _confirm: String = read!("{}\n");
        let out_bridge = bridge.register_user(&username).unwrap();
        match file.write_all(out_bridge.username.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
        return out_bridge;
    }
}
