use std::collections::HashMap;
use crate::input::keypoint::Keypoint;

// Creating a hashmap the takes in a keypoint name.
// And returns the row/col number in the global stiffness matrix.
// Which is the same row in the force and displacement vector.
// Note that only the first degree of freedom number is returned.
pub fn global_stiffness_matrix_keypoint_hashmap(kp_list: &[Keypoint]) -> HashMap<String, usize> {
	let mut kp_map: HashMap<String, usize> = HashMap::new();	
	let mut number: usize = 0;

	for kp in kp_list {
		kp_map.insert(kp.name.clone(), number);
		number += 2;
	}

	kp_map
}