use std::fs;

#[derive(Debug)]
pub struct Pointload {
	pub name: String,
	pub keypoint: String,
	pub load_x: f64,
	pub load_y: f64,
}

pub fn parse_pointload(file_path: &str) -> Vec<Pointload> {
	let mut pointloads: Vec<Pointload> = Vec::new();

	let content: String = fs::read_to_string(file_path).unwrap();

	for line in content.lines() {

		if line.starts_with('#') {
			continue;
		}

		let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

		let name: String = parts[0].to_string();
		let keypoint: String = parts[1].to_string();
		let load_x: f64 = parts[2].parse().unwrap();
		let load_y: f64 = parts[3].parse().unwrap();

		pointloads.push(Pointload {name, keypoint, load_x, load_y});
	}

	pointloads
}