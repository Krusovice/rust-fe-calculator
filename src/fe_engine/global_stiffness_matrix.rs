// This file contains generation of the stiffness matrix.

use nalgebra::{DMatrix, DVector};
use crate::input::keypoint::Keypoint;
use crate::input::connection::Connection;
use crate::material_formulation::local_stiffness_matrix_bar::local_bar_matrix;
use std::collections::HashMap;

pub fn create_global_stiffness_matrix(kp_list: &[Keypoint], conn_list: &[Connection], E:f64, A:f64) -> DMatrix<f64> {
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
		// Finding the keypoints structs needed for calculating the local stiffness matrix.
		let kp_1 = kp_list.iter().find(|kp| kp.name == conn.kp_1).unwrap();
		let kp_2 = kp_list.iter().find(|kp| kp.name == conn.kp_2).unwrap();
		let local_bar_mat:DMatrix<f64> = local_bar_matrix(kp_1, kp_2, E, A);

		// Finding keypoint locations in the global stiffness matrix.
		let loc_1 = kp_map[&conn.kp_1];
		let loc_2 = kp_map[&conn.kp_2];

		// Inserting keypoint local values into the global stiffness matrix.
		global_identity_matrix[(loc_1,loc_1)] += local_bar_mat[(0,0)];
		global_identity_matrix[(loc_1,loc_1+1)] += local_bar_mat[(0,1)];
		global_identity_matrix[(loc_1+1,loc_1)] += local_bar_mat[(1,0)];
		global_identity_matrix[(loc_1+1,loc_1+1)] += local_bar_mat[(1,1)];

		global_identity_matrix[(loc_2,loc_2)] += local_bar_mat[(2,2)];
		global_identity_matrix[(loc_2,loc_2+1)] += local_bar_mat[(2,3)];
		global_identity_matrix[(loc_2+1,loc_2)] += local_bar_mat[(3,2)];
		global_identity_matrix[(loc_2+1,loc_2+1)] += local_bar_mat[(3,3)];

		global_identity_matrix[(loc_1,loc_2)] += local_bar_mat[(0,2)];
		global_identity_matrix[(loc_1,loc_2+1)] += local_bar_mat[(0,3)];
		global_identity_matrix[(loc_1+1,loc_2)] += local_bar_mat[(1,2)];
		global_identity_matrix[(loc_1+1,loc_2+1)] += local_bar_mat[(1,3)];

		global_identity_matrix[(loc_2,loc_1)] += local_bar_mat[(2,0)];
		global_identity_matrix[(loc_2,loc_1+1)] += local_bar_mat[(3,0)];
		global_identity_matrix[(loc_2+1,loc_1)] += local_bar_mat[(2,1)];
		global_identity_matrix[(loc_2+1,loc_1+1)] += local_bar_mat[(3,1)];

	}

	global_identity_matrix
}


// Applying the boundary conditions to the global stiffnessmatrix.
// Returning the modified global stiffness matrix.
// Inserting mut global stiffness matrix, applying new owner.
pub fn apply_boundary_conditions(global_stiffness_matrix:&DMatrix<f64>, dof_filter_vector:&DVector<f64>) -> DMatrix<f64> {
	let mut modified_global_stiffness_matrix:DMatrix<f64> = global_stiffness_matrix.clone();

	let size = dof_filter_vector.nrows();

	for i in 0..size {
		if dof_filter_vector[i] == 0.0 {
			for j in 0..size {
				modified_global_stiffness_matrix[(i,j)] = 0.0;
				modified_global_stiffness_matrix[(j,i)] = 0.0;
			}
			modified_global_stiffness_matrix[(i,i)] = 1.0;
		}
	}

	modified_global_stiffness_matrix
}

// This functions calculates the displacement vector by inverting the modified global stiffness matrix.
// Before inverting, all dof's (rows and columns) with fixed boundary conditions (diagonal elements = 1)
// are removed. That for both vector and global stiffness matrix.
// After inverting, the global stiffness matrix is assembled again, by re-adding the removed dof's.
pub fn calculate_displacement_vector(modified_global_stiffness_matrix:&DMatrix<f64>, force_vector:&DVector<f64>) -> DVector<f64> {
	// Applying nalgebra's lu solver to solve a linear matrix system of matrix*vector = vector.
	let lu = modified_global_stiffness_matrix.clone().lu();
	let displacement_vector = lu.solve(force_vector).unwrap();

	displacement_vector
}



