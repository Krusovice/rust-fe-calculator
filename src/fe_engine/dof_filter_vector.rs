// This file contains a function for implementing
// boundary conditions to the global stiffness matrix.

use nalgebra::{DVector};
use crate::input::keypoint::Keypoint;
use crate::input::boundary_condition::BoundaryCondition;
use std::collections::HashMap;

pub fn create_dof_filter_vector(kp_list:&[Keypoint], bc_list:&[BoundaryCondition]) -> DVector<f64> {
	// Creating dict for keypoint force directions and global force vector location.
	let mut kp_map:HashMap<String, usize> = HashMap::new();
	let mut number:usize = 0;

	for kp in kp_list {
		kp_map.insert(format!("{}_x", kp.name), number);
		kp_map.insert(format!("{}_y", kp.name), number+1);
		number += 2;
	}

	// Creating vector size based on bar elements.
	let size: usize = 2*kp_list.len();

	// Creating boundary condition vector.
	let mut dof_filter_vec:DVector<f64> = DVector::from_element(size, 1.0);

	// Modifying our boundary condition vector, to get zeros at each fixed boundary condition.
	for bc in bc_list {
		if bc.fixture == "0" {
			let loc_x:usize = *kp_map.get(&format!("{}_x", bc.keypoint)).unwrap();
			dof_filter_vec[loc_x] = 0.0;
		}
		else if bc.fixture == "1" {
			let loc_y:usize = *kp_map.get(&format!("{}_y", bc.keypoint)).unwrap();
			dof_filter_vec[loc_y] = 0.0;
		}
		else if bc.fixture == "2" {
			let loc_x:usize = *kp_map.get(&format!("{}_x", bc.keypoint)).unwrap();
			dof_filter_vec[loc_x] = 0.0;
			let loc_y:usize = *kp_map.get(&format!("{}_y", bc.keypoint)).unwrap();
			dof_filter_vec[loc_y] = 0.0;
		}
	}

	dof_filter_vec
}