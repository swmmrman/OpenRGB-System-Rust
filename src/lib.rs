use std::fs;
use std::io;

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
