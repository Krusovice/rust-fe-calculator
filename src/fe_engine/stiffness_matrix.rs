// This file contains generation of the stiffness matrix.

use nalgebra::DMatrix;
use crate::input::keypoint::Keypoint;

pub fn create_global_unit_matrix(kp_list: &[Keypoint]) -> DMatrix<f64> {
	// Creating size based on bar elements
	let size: usize = 2*kp_list.len();

	// Making an identity matrix based on number of keypoints
	let global_identitity_matrix = DMatrix::<f64>::identity(size, size);

	global_identitity_matrix
}



