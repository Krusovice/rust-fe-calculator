use std::fs;

#[derive(Debug)]
pub struct BoundaryCondition {
	pub name: String,
	pub keypoint: String,
	pub fixture: String, // 0=x-direction, 1=y-direction, 2=both x and y-direction.
	pub spring_stiffness: f64 // -1 is a fixed bc. Other values are the spring value.
}

pub fn parse_boundary_condition(file_path: &str) -> Vec<BoundaryCondition> {
	let mut bcs: Vec<BoundaryCondition> = Vec::new();

	let content: String = fs::read_to_string(file_path).unwrap();

	for line in content.lines() {
		if line.starts_with('#') {
			continue;
		}

		let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

		let name: String = parts[0].to_string();
		let keypoint: String = parts[1].to_string();
		let fixture: String = parts[2].to_string();
		let spring_stiffness: f64 = parts[3].parse().unwrap(); // parse as f64
		

		bcs.push(BoundaryCondition {name, keypoint, fixture, spring_stiffness});
	}

	bcs
}