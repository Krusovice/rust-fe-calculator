use std::fs;

#[derive(Debug)]
pub struct Boundary_condition {
	pub name: String,
	pub keypoint: String,
	pub fixture: String, // 0=x-direction, 1=y-direction, 2=both x and y-direction
}

pub fn parse_boundary_condition(file_path: &str) -> Vec<Boundary_condition> {
	let mut bcs: Vec<Boundary_condition> = Vec::new();

	let content: String = fs::read_to_string(file_path).unwrap();

	for line in content.lines() {
		if line.starts_with('#') {
			continue;
		}

		let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

		let name: String = parts[0].to_string();
		let keypoint: String = parts[1].to_string();
		let fixture: String = parts[2].to_string();

		bcs.push(Boundary_condition {name, keypoint, fixture});
	}

	bcs
}