use std::fs;

#[derive(Debug)]
pub struct Material {
	pub name: String,
	pub e_modulus: f64,
}

pub fn parse_material(file_path: &str) -> Vec<Material> {
	let mut materials: Vec<Material> = Vec::new();

	let content: String = fs::read_to_string(file_path).unwrap();

	for line in content.lines() {
		if line.starts_with('#') {
			continue;
		}

		let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

		let name: String = parts[0].to_string();
		let e_modulus: f64 = parts[1].parse().unwrap();

		materials.push(Material {name, e_modulus});
	}

	materials
}