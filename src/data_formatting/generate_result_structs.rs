use crate::input::keypoint::Keypoint;
use nalgebra::DVector;
use crate::fe_engine::utils::global_stiffness_matrix_keypoint_hashmap;

pub fn generate_result_keypoint(kp_list:&mut [Keypoint], 
								force_vector:&DVector<f64>,
								displacement_vector:&DVector<f64>) {

	let kp_hashmap = global_stiffness_matrix_keypoint_hashmap(&kp_list);

	for kp in kp_list.iter_mut() {
		let number = kp_hashmap[&kp.name];
		kp.fx = force_vector[number];
		kp.fy = force_vector[number+1];
		kp.ux = displacement_vector[number];
		kp.uy = displacement_vector[number+1];
	}
}