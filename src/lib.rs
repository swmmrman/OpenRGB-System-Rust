use std::fs;
use std::io;
use openrgb::{
    data::Color,
    data::LED,
};
use rgb;

pub fn get_cpu_file() -> Result<String, io::Error> {
    let mut cpufile = String::new();
    let zones = fs::read_dir("/sys/class/thermal/")?;
    for zone in zones {
        let type_path = format!("{}/type", &zone.as_ref().unwrap().path().display());
        let sensor_type = fs::read_to_string(type_path).expect("Error");
        if sensor_type == "x86_pkg_temp\n" {
            cpufile = format!("{}/temp", &zone.as_ref().unwrap().path().display());
        }
    }
    Ok(cpufile)
}

pub fn get_cpu_temp(path: &str) -> f32 {
    let temp: String = fs::read_to_string(path)
        .expect(&format!("File not found {}", path));
    let temp = temp.trim().parse::<f32>().unwrap();
    temp / 1000.0
}

pub fn get_color(val: f32) -> Color {
    if val < 0.01{ 
        return Color::new(0,0,0);
    }
    let val = val * 2.0;
    let r = std::cmp::max(std::cmp::min((val * 255.0) as u8, 255), 0);
    let g = std::cmp::max(std::cmp::min((510.0 - val * 255.0) as u8, 255), 0);
    let b = 0;
    Color::new(r,g,b)
}

pub fn get_key_indexs(keys: Vec<&str>, leds: &Vec<LED>) -> Vec<usize> {
    let mut indexs = Vec::new();
    let mut led_names = Vec::new();
    for led in leds {
        let led_name = led.name.to_string();
        if led_name == "Logo" {
            led_names.push(led_name);
        }
        else {
            led_names.push(led_name[5..].to_string());
        }
    }
    for key in keys {
        let index = led_names.iter().position(|x| x == &key.to_string()).unwrap();
        indexs.push(index);
    }
    indexs
}
pub fn get_fans() -> Vec<f32> {
    let mut fans: Vec<f32>= Vec::new();
    let sensor_packs = fs::read_dir("/sys/class/hwmon/").unwrap();
    for sensor in sensor_packs {
        let sensor_name_file = format!("{}/name", &sensor.as_ref().unwrap().path().display()); 
        let sensor_name = fs::read_to_string(sensor_name_file).expect("File not found");
        if sensor_name == "nct6687\n" {
            for i in 1..=8 {
                let fan_path = format!("{}/fan{}_input", &sensor.as_ref().unwrap().path().display(), i);
                let fanspeed = fs::read_to_string(fan_path).unwrap();
                fans.push(fanspeed.trim().parse::<f32>().unwrap());
            }
            break;
        }
    }
    fans
}

pub fn get_fan_colors(colors: &mut Vec<rgb::RGB<u8>>, indexs: &Vec<usize>) {
    for (i, fan) in get_fans().iter().enumerate() {
        let max_speeds = vec!(2250.0, 4800.0, 2000.0, 2250.0, 2250.0, 2200.0, 2200.0, 2200.0);
        let fan_led = indexs[i + 21];
        let fan_percent: f32 = fan / max_speeds[i];
        colors[fan_led] = get_color(fan_percent);
    }
}