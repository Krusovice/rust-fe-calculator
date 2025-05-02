use serde::Serialize;
use crate::input::keypoint::Keypoint;
use std::fs::File;
use std::io::{Write, Result};

pub fn eksport_keypoint_structs(kp_list: &[Keypoint],
								output_path: &str,) {
	let json_string = serde_json::to_string_pretty(kp_list).unwrap();
    // Create and write to the file
    let mut file = File::create(output_path).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
}
