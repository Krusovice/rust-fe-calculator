// This file contains generation of the stiffness matrix.

use nalgebra::{DMatrix, DVector};
use crate::input::keypoint::Keypoint;
use crate::input::connection::Connection;
use crate::material_formulation::local_stiffness_matrix_bar::local_bar_matrix;
use crate::fe_engine::utils::global_stiffness_matrix_keypoint_hashmap;
use std::collections::HashMap;

pub fn create_global_stiffness_matrix(kp_list: &[Keypoint], 
									  conn_list: &[Connection], 
									  e_module:f64, 
									  area:f64
									  ) -> DMatrix<f64> {
	
	// Creating size based on bar elements
	let size: usize = 2*kp_list.len();

	// Making an identity matrix based on number of keypoints
	let mut global_identity_matrix = DMatrix::<f64>::zeros(size, size);

	for conn in conn_list {
		// Finding the keypoints structs needed for calculating the local stiffness matrix.
		let kp_1 = kp_list.iter().find(|kp| kp.name == conn.kp_1).unwrap();
		let kp_2 = kp_list.iter().find(|kp| kp.name == conn.kp_2).unwrap();
		let local_bar_mat:DMatrix<f64> = local_bar_matrix(kp_1, kp_2, e_module, area);

		// Finding keypoint locations in the global stiffness matrix.
		let kp_map = global_stiffness_matrix_keypoint_hashmap(&kp_list);
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

	let size:usize = dof_filter_vector.nrows();

	for i in 0..size {
		if dof_filter_vector[i] == 0.0 {
			for j in 0..size {
				modified_global_stiffness_matrix[(i,j)] = 0.0;
				modified_global_stiffness_matrix[(j,i)] = 0.0;
			}
			modified_global_stiffness_matrix[(i,i)] = 1.0;
		}

		else if dof_filter_vector[i] > 0.0 {
			modified_global_stiffness_matrix[(i,i)] += dof_filter_vector[i];
		}
	}
	modified_global_stiffness_matrix
}

// This functions calculates the displacement vector by inverting the modified global stiffness matrix.
// Before inverting, all dof's (rows and columns) with fixed boundary conditions (diagonal elements = 1).
// are removed. That for both vector and global stiffness matrix.
// After inverting, the global stiffness matrix is assembled again, by re-adding the removed dof's.
pub fn calculate_resulting_displacement_vector(modified_global_stiffness_matrix:&DMatrix<f64>, force_vector:&DVector<f64>, dof_filter_vector:&DVector<f64>) -> DVector<f64> {

	// Creating a hashmap for correlating matrix locations for global and reduced stiffness matrices.
	// The key is the total stiffness matrix location. The value is the reduced stiffness matrix location.
	// A value of -999 is set for a non-existent location in the reduced stiffness matrix.
	let mut global_stiffness_matrix_map:HashMap<usize, i32> = HashMap::new();

	let size:usize = force_vector.nrows();
	let mut loc_reduced:i32 = 0;
	for loc_global in 0..size {
		if dof_filter_vector[loc_global] == 0.0 {
			global_stiffness_matrix_map.insert(loc_global,-999);
		}
		else {
			global_stiffness_matrix_map.insert(loc_global,loc_reduced);
			loc_reduced += 1;
		}
	}

	// Creating the reduced stiffness matrix and force vector.
	// The reduced stiffness matrix and force vector contains only locations with known forces and unknown dispacements.
	// That means that each column and row that contains a zero, is removed.
	// That is the reduced stiffness matrix. The same rows are removed for the force vector.
	let size_reduced = global_stiffness_matrix_map.values().filter(|&&v| v != -999).count();


	let mut modified_global_stiffness_matrix_reduced:DMatrix<f64> = DMatrix::<f64>::zeros(size_reduced,size_reduced);
	let mut force_vector_reduced:DVector<f64> = DVector::<f64>::zeros(size_reduced);

	for i in 0..size {
		if global_stiffness_matrix_map[&i] == -999 {
			continue;
		}
		let reduced_loc_row = global_stiffness_matrix_map[&i];
		for j in 0..size {
			if global_stiffness_matrix_map[&j] == -999 {
				continue;
			}
			let reduced_loc_col = global_stiffness_matrix_map[&j];
			modified_global_stiffness_matrix_reduced[(reduced_loc_row as usize,reduced_loc_col as usize)]
			= modified_global_stiffness_matrix[(i as usize,j as usize)];
		}
		force_vector_reduced[reduced_loc_row as usize] = force_vector[i];
	}

	// The displacements (u) at location with no bc's are found
	// by solving the reduced stiffness matrix and reduced force vector. u*K=F.
	// Applying nalgebra's lu solver to solve.
	let lu = modified_global_stiffness_matrix_reduced.clone().lu();
	let displacement_vector_reduced = lu.solve(&force_vector_reduced).unwrap();

	// The global stiffness matrix to reduced stiffness matrix hashmap is now used to insert the displacement values at correct locations.
	let mut displacement_vector:DVector<f64> = DVector::<f64>::zeros(size);

	let mut loc_reduced:i32 = 0;
	for i in 0..size {
		if global_stiffness_matrix_map[&i] == -999 {
			displacement_vector[i as usize] = 0.0;
		}
		else {
			let u_value:f64 = displacement_vector_reduced[loc_reduced as usize];
			displacement_vector[i as usize] = u_value;
			loc_reduced += 1;
		}
	}

	displacement_vector
}

pub fn calculate_resulting_force_vector(
	global_stiffness_matrix:&DMatrix<f64>, 
	resulting_displacement_vector:&DVector<f64>) -> DVector<f64> {

	let resulting_force_vector:DVector<f64> = global_stiffness_matrix * resulting_displacement_vector;

	resulting_force_vector
}

