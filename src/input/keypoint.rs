use std::fs;

// Structure for keypoint objects.
#[derive(Debug)]
pub struct Keypoint {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

pub fn parse_keypoint(file_path: &str) -> Vec<Keypoint> {
    let mut keypoints: Vec<Keypoint> = Vec::new();

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

        keypoints.push(Keypoint { name, x, y });

    }

    keypoints
}