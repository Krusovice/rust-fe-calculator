//use crate::input::material::Material;
use nalgebra::{DMatrix, Vector2};
use crate::input::keypoint::Keypoint;


/// Calculates the local stiffness matrix for a 2D bar element.
///
/// # Arguments
/// * `E` - Young's modulus of the material
/// * `A` - Cross-sectional area of the element
/// * `kp_1` - Position vector of the first keypoint
/// * `kp_2` - Position vector of the second keypoint
///
/// # Returns
/// * `DMatrix<f64>` - A 4x4 local stiffness matrix in global coordinates
pub fn local_bar_matrix(kp_1:&Keypoint, kp_2:&Keypoint, e_module:f64, area:f64) -> DMatrix<f64> {
    // Creating vectors of kp_1 and kp_2
    let vec_kp_1 = Vector2::new(kp_1.x, kp_1.y);
    let vec_kp_2 = Vector2::new(kp_2.x, kp_2.y);

    // delta vector between the two kp inputs.
    let vec_delta:Vector2<f64> = vec_kp_2 - vec_kp_1;

    // Vector length.
    let length:f64 = vec_delta.norm();

    // delta.x over length = cosinus(theta)
    let c:f64 = vec_delta.x / length;

    // delta.y over length = sinus(theta)
    let s:f64 = vec_delta.y / length;

    // Constructing the local element matrix for a bar
    let mut k = DMatrix::<f64>::zeros(4, 4);

    k[(0, 0)] =  c * c;
    k[(0, 1)] =  c * s;
    k[(0, 2)] = -c * c;
    k[(0, 3)] = -c * s;

    k[(1, 0)] =  c * s;
    k[(1, 1)] =  s * s;
    k[(1, 2)] = -c * s;
    k[(1, 3)] = -s * s;

    k[(2, 0)] = -c * c;
    k[(2, 1)] = -c * s;
    k[(2, 2)] =  c * c;
    k[(2, 3)] =  c * s;

    k[(3, 0)] = -c * s;
    k[(3, 1)] = -s * s;
    k[(3, 2)] =  c * s;
    k[(3, 3)] =  s * s;

    k * e_module * area / length
}
