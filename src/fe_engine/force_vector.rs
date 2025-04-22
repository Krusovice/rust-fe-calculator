// This file creates the force vector based on pointload inputs.

use nalgebra::DVector;
use std::collections::HashMap;
use crate::input::keypoint::Keypoint;
use crate::input::pointload::Pointload;

pub fn create_force_vector(kp_list:&[Keypoint], pl_list:&[Pointload]) -> DVector<f64> {
	// Creating dict for keypoint force directions and global force vector location.
	let mut kp_map:HashMap<String, usize> = HashMap::new();
	let mut number:usize = 0;

	for kp in kp_list {
		kp_map.insert(format!("{}_x", kp.name), number);
		kp_map.insert(format!("{}_y", kp.name), number+1);
		number += 2;
	}

	// Creating vector size based on bar elements.
	let size:usize = 2*kp_list.len();

	// Creating force vector.
	let mut pl_vec:DVector<f64> = DVector::from_element(size, 0.0);

	for pl in pl_list {
		let loc_x:usize = *kp_map.get(&format!("{}_x",pl.keypoint)).unwrap();
		pl_vec[loc_x] = pl.load_x;
		let loc_y:usize = *kp_map.get(&format!("{}_y",pl.keypoint)).unwrap();
		pl_vec[loc_y] = pl.load_y;
	}
	
	pl_vec
} 