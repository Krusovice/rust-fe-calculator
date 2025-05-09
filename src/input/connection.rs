use std::fs;

#[derive(Debug)]
pub struct Connection {
	pub name: String,
	pub kp_1: String,
	pub kp_2: String,
	pub material: String,
}

pub fn parse_connection(file_path: &str) -> Vec<Connection> {
	let mut connections: Vec<Connection> = Vec::new();

	let content: String = fs::read_to_string(file_path).unwrap();

	for line in content.lines() {

		if line.starts_with('#') {
			continue;
		}

		let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

		let name: String = parts[0].to_string();
		let kp_1: String = parts[1].to_string();
		let kp_2: String = parts[2].to_string();
		let material: String = parts[3].to_string();

		connections.push(Connection {name, kp_1, kp_2, material});
	}

	connections
}