use std::fs;

// Structure for keypoint objects.
#[derive(Debug)]
pub struct Keypoint {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub fx: f64,
    pub fy: f64,
    pub ux: f64,
    pub uy: f64,
}

pub fn parse_keypoint(file_path: &str) -> Vec<Keypoint> {
    let mut kp_list: Vec<Keypoint> = Vec::new();

    let content: String = fs::read_to_string(file_path).unwrap();

    // Iterate through lines of input file.
    for line in content.lines() {

        // Skip line if it starts with hashtag.
        if line.starts_with('#') {
            continue;
        }

        // Parse the line into keypoint struct.
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        let name = parts[0].to_string();
        let x = parts[1].parse::<f64>().unwrap();
        let y = parts[2].parse::<f64>().unwrap();

        kp_list.push(Keypoint { name, x, y, fx:0.0, fy:0.0, ux:0.0, uy:0.0});

    }

    kp_list
}