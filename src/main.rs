mod input {
    pub mod keypoint;
    pub mod connection;
    pub mod boundary_condition;
    pub mod pointload;
    //pub mod material;
}

mod fe_engine {
    pub mod global_stiffness_matrix;
    pub mod dof_filter_vector;
    pub mod force_vector;
    pub mod utils;
}

mod material_formulation {
    pub mod local_stiffness_matrix_bar;
}

mod output {
    pub mod canvas;
    pub mod figures;
    pub mod content_and_labels;
    
}

mod data_formatting {
    pub mod generate_result_structs;
}

use input::keypoint::{parse_keypoint};
use input::connection::{parse_connection};
use input::boundary_condition::{parse_boundary_condition};
use input::pointload::{parse_pointload};
//use input::material::{parse_material, Material};
use fe_engine::global_stiffness_matrix::{
    create_global_stiffness_matrix,
    apply_boundary_conditions, 
    calculate_resulting_displacement_vector,
    calculate_resulting_force_vector};
use fe_engine::dof_filter_vector::{create_dof_filter_vector};
use fe_engine::force_vector::{create_force_vector};
use output::figures::{geometry_plot, reaction_plot};
use data_formatting::generate_result_structs::{generate_result_keypoint};

// Hardcoding material parameters, 
// A=Area
// E=Stiffness
const MATERIAL_AREA: f64 = 0.1;
const MATERIAL_E_MODULE: f64 = 210000.0;
const PLOT_GEOMETRY_OUTPUT_PATH: &str = "outputs/geometry_plot.png";
const PLOT_REACTION_OUTPUT_PATH: &str = "outputs/reaction_plot.png";
const PLOT_DIMENSION: (u32, u32) = (800, 300);
const PLOT_RESULT_SCALE: f32 = 1.0;
const PLOT_FEATURE_SIZE: f32 = 2.0;
const PLOT_RESULT_DECIMALS: usize = 2;

fn main() {
    let mut kp_list = parse_keypoint("inputs/keypoints.txt");
    let conn_list = parse_connection("inputs/connections.txt");
    let bc_list = parse_boundary_condition("inputs/bcs.txt");
    let pl_list = parse_pointload("inputs/pointloads.txt");
    //let mat_list = parse_material("inputs/materials.txt");

    println!("Parsed Keypoints:\n{:#?}", kp_list);
    println!("Parsed Connections:\n{:#?}", conn_list);
    println!("Parsed Boundary Conditions:\n{:#?}", bc_list);
    println!("Parsed Pointloads:\n{:#?}", pl_list);
    //println!("Parsed Materials:\n{:#?}", mat_list);

    let _ = geometry_plot(&kp_list, 
                          &conn_list, 
                          &bc_list, 
                          &pl_list, 
                          PLOT_FEATURE_SIZE,
                          &PLOT_GEOMETRY_OUTPUT_PATH,
                          PLOT_DIMENSION,
                          &"Geometry Plot");

    let global_stiffness_matrix = create_global_stiffness_matrix(&kp_list, &conn_list, MATERIAL_E_MODULE, MATERIAL_AREA);
    println!("Global stiffness matrix:\n{}", global_stiffness_matrix);

    let dof_filter_vector = create_dof_filter_vector(&kp_list, &bc_list);
    println!("DOF filter vector:\n{}", dof_filter_vector);

    let force_vector = create_force_vector(&kp_list, &pl_list);
    println!("Force vector:\n{}", force_vector);



    let modified_global_stiffness_matrix = apply_boundary_conditions(&global_stiffness_matrix, &dof_filter_vector);
    println!("Modified global stiffness matrix:\n{}", modified_global_stiffness_matrix);

    let resulting_displacement_vector = calculate_resulting_displacement_vector(&modified_global_stiffness_matrix, &force_vector, &dof_filter_vector);
    println!("Resulting Displacement Vector:\n{}", resulting_displacement_vector);

    let resulting_force_vector = calculate_resulting_force_vector(&global_stiffness_matrix, &resulting_displacement_vector);
    println!("Resulting Force Vector:\n{}", resulting_force_vector);

    generate_result_keypoint(&mut kp_list, &resulting_force_vector, &resulting_displacement_vector);
    println!("Resulting keypoint forces and displacements:\n{:#?}", kp_list);

    let _ = reaction_plot(&kp_list, 
                          &conn_list, 
                          &bc_list, 
                          &pl_list, 
                          PLOT_FEATURE_SIZE,
                          &PLOT_REACTION_OUTPUT_PATH,
                          PLOT_DIMENSION,
                          &"Reaction Plot",
                          PLOT_RESULT_SCALE,
                          PLOT_RESULT_DECIMALS);
}