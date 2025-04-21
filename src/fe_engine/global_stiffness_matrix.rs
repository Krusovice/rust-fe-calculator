// This file contains generation of the stiffness matrix.

use nalgebra::DMatrix;
use crate::input::keypoint::Keypoint;
use crate::input::connection::Connection;
use std::collections::HashMap;

pub fn create_global_unit_matrix(kp_list: &[Keypoint], conn_list: &[Connection]) -> DMatrix<f64> {
	// Creating size based on bar elements
	let size: usize = 2*kp_list.len();

	// Making an identity matrix based on number of keypoints
	let mut global_identity_matrix = DMatrix::<f64>::zeros(size, size);

	// Creating a dictionary for all keypoints, to associate 
	// keypoint with global stiffness matrix location
	let mut kp_map: HashMap<String, usize> = HashMap::new();	
	let mut number: usize = 0;

	for kp in kp_list {
		kp_map.insert(kp.name.clone(), number);
		number += 2;
	}

	for conn in conn_list {
		let kp_1 = &conn.kp_1;
		let kp_2 = &conn.kp_2;

		let loc_1 = kp_map[kp_1];
		let loc_2 = kp_map[kp_2];

		global_identity_matrix[(loc_1,loc_1)] += 1.0;
		global_identity_matrix[(loc_1+1,loc_1+1)] += 1.0;
		global_identity_matrix[(loc_2,loc_2)] += 1.0;
		global_identity_matrix[(loc_2+1,loc_2+1)] += 1.0;
		global_identity_matrix[(loc_1,loc_2)] += -1.0;
		global_identity_matrix[(loc_1+1,loc_2+1)] += -1.0;
		global_identity_matrix[(loc_2,loc_1)] += -1.0;
		global_identity_matrix[(loc_2+1,loc_1+1)] += -1.0;
	}

	global_identity_matrix
}



