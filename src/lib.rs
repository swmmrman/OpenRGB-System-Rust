use std::fs;
use std::io;
use openrgb::{
    data::Color,
    data::LED,
};
use rgb;
use std::collections::VecDeque;
use std::path::PathBuf;

pub fn get_cpu_file() -> Result<PathBuf, io::Error> {
    let mut cpufile = PathBuf::new();
    let zones = fs::read_dir("/sys/class/thermal/")?;
    for zone in zones {
        let type_path = &zone.as_ref().unwrap().path().join("type");
        let sensor_type = fs::read_to_string(type_path).expect("Error");
        if sensor_type == "x86_pkg_temp\n" {
            //cpufile = format!("{}/temp", &zone.as_ref().unwrap().path().display());
            cpufile = zone.unwrap().path().join("temp");
        }
    }
    Ok(cpufile)
}

pub fn get_cpu_temp(path: &PathBuf) -> f32 {
    let temp: String = fs::read_to_string(path)
        .expect(&format!("File not found {}", path.display()));
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
    //println!("{:?}", &leds);
    for led in leds {
        let led_name = led.name.to_string();
        if led_name == "" {
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

pub fn get_fan_colors(mut colors: Vec<rgb::RGB<u8>>, indexs: &Vec<usize>) -> Vec<rgb::RGB<u8>> {
    for (i, fan) in get_fans().iter().enumerate() {
        let max_speeds = vec!(2250.0, 4800.0, 2000.0, 2250.0, 2250.0, 2200.0, 2200.0, 2200.0);
        let fan_led = indexs[i + 21];
        let fan_percent: f32 = fan / max_speeds[i];
        colors[fan_led] = get_color(fan_percent);
    }
    colors.to_vec()
}

pub fn get_cpu_avg(cpu_vals: &mut VecDeque<f32>, cpu_file: &PathBuf) -> f32{
    cpu_vals.pop_front();
    cpu_vals.push_back(((get_cpu_temp(&cpu_file) - 24.0)*1.4) / 100.0);
    cpu_vals.iter().sum::<f32>() / 10.0
}

#[cfg(test)]
mod test {
    use std::{collections::VecDeque, path::{Path, PathBuf}};
    #[test]
    fn test_cpu_avg(){
        let mut vals: VecDeque<f32> =  [ 0.23, 0.25, 0.1, 0.2, 0.5, 0.10, 0.8, 0.2, 0.4, 0.8 ].into();
        let targ = PathBuf::new().join("cpu_fake_file");
        assert_eq!(super::get_cpu_avg(&mut vals, &targ), 0.36034003);
    }
    #[test]
    fn test_cpu_temp() {
        let targ = PathBuf::new().join("cpu_fake_file");
        assert_eq!(super::get_cpu_temp(&targ), 42.1);
    }
    #[test]
    fn test_cpu_file() {
        let f = super::get_cpu_file().unwrap();
        let p_f = Path::new(&f).parent().unwrap();
        let t_f = p_f.join("type");
        let m_f = p_f.join("mode");
        assert_eq!(m_f.display().to_string(), "/sys/class/thermal/thermal_zone1/mode");
        assert_eq!(std::fs::read_to_string(t_f).unwrap(), "x86_pkg_temp\n");
        assert_eq!(std::fs::read_to_string(m_f).unwrap(), "enabled\n");
        let old_temp = std::fs::read_to_string(&f).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_ne!(std::fs::read_to_string(&f).unwrap(), old_temp)
    }
}