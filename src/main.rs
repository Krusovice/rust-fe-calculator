mod input {
    pub mod keypoint;
    pub mod connection;
    pub mod boundary_condition;
    pub mod pointload;
    pub mod material;
}

mod fe_engine {
    pub mod stiffness_matrix;
}

use input::keypoint::{parse_keypoint, Keypoint};
use input::connection::{parse_connection, Connection};
use input::boundary_condition::{parse_boundary_condition, Boundary_condition};
use input::pointload::{parse_pointload, Pointload};
use input::material::{parse_material, Material};
use fe_engine::stiffness_matrix::{create_global_unit_matrix};

fn main() {
    let kp_list = parse_keypoint("inputs/keypoints.txt");
    let conn_list = parse_connection("inputs/connections.txt");
    let bc_list = parse_boundary_condition("inputs/bcs.txt");
    let pl_list = parse_pointload("inputs/pointloads.txt");
    let mat_list = parse_material("inputs/materials.txt");

    println!("Parsed Keypoints:\n{:#?}", kp_list);
    println!("Parsed Connections:\n{:#?}", conn_list);
    println!("Parsed Boundary Conditions:\n{:#?}", bc_list);
    println!("Parsed Pointloads:\n{:#?}", pl_list);
    println!("Parsed Materials:\n{:#?}", mat_list);

    let global_identitity_matrix = create_global_unit_matrix(&kp_list);
    println!("Global identity matrix:\n{}", global_identitity_matrix)
}