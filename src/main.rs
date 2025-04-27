mod input {
    pub mod keypoint;
    pub mod connection;
    pub mod boundary_condition;
    pub mod pointload;
    pub mod material;
}

mod fe_engine {
    pub mod global_stiffness_matrix;
    pub mod dof_filter_vector;
    pub mod force_vector;
}

mod material_formulation {
    pub mod local_stiffness_matrix_bar;
}

mod output {
    pub mod plots;
}

use input::keypoint::{parse_keypoint, Keypoint};
use input::connection::{parse_connection, Connection};
use input::boundary_condition::{parse_boundary_condition, BoundaryCondition};
use input::pointload::{parse_pointload, Pointload};
use input::material::{parse_material, Material};
use fe_engine::global_stiffness_matrix::{
    create_global_stiffness_matrix,
    apply_boundary_conditions, 
    calculate_resulting_displacement_vector,
    calculate_resulting_force_vector};
use fe_engine::dof_filter_vector::{create_dof_filter_vector};
use fe_engine::force_vector::{create_force_vector};
use output::plots::{reaction_plot};

// Hardcoding material parameters, 
// A=Area
// E=Stiffness
const MATERIAL_A: f64 = 5000.0;
const MATERIAL_E: f64 = 0.1;

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

    let global_stiffness_matrix = create_global_stiffness_matrix(&kp_list, &conn_list, MATERIAL_E, MATERIAL_A);
    println!("Global stiffness matrix:\n{}", global_stiffness_matrix);

    let dof_filter_vector = create_dof_filter_vector(&kp_list, &bc_list);
    println!("DOF filter vector:\n{}", dof_filter_vector);

    let force_vector = create_force_vector(&kp_list, &pl_list);
    println!("Force vector:\n{}", force_vector);

    let modified_global_stiffness_matrix = apply_boundary_conditions(&global_stiffness_matrix, &dof_filter_vector);
    println!("Modified global stiffness matrix:\n{}", modified_global_stiffness_matrix);

    let resulting_displacement_vector = calculate_resulting_displacement_vector(&modified_global_stiffness_matrix, &force_vector);
    println!("Resulting Displacement Vector:\n{}", resulting_displacement_vector);

    let resulting_force_vector = calculate_resulting_force_vector(&global_stiffness_matrix, &resulting_displacement_vector);
    println!("Resulting Force Vector:\n{}", resulting_force_vector);

    reaction_plot(&kp_list);
}