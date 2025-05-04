use crate::input::keypoint::Keypoint;
use crate::input::connection::Connection;
use crate::input::boundary_condition::BoundaryCondition;
use crate::input::pointload::Pointload;
use crate::output::canvas::plot_canvas;
use crate::output::content_and_labels::
{plot_keypoint, plot_keypoint_displaced, plot_connection, 
plot_connection_displaced, plot_boundary_condition, plot_pointload};

pub fn geometry_plot(kp_list:&[Keypoint], 
                     conn_list:&[Connection], 
                     bc_list:&[BoundaryCondition], 
                     pl_list:&[Pointload], 
                     plot_feature_size:f32,
                     output_path:&str,
                     dimension:(u32, u32),
                     chart_title:&str) -> Result<(), Box<dyn std::error::Error>> {

    // Creating the plotting canvas, returning the struct "chart_context"
    let mut chart_context = plot_canvas(&kp_list, &output_path, dimension, &chart_title);

    for kp in kp_list {
        plot_keypoint(&mut chart_context, &kp, plot_feature_size);
    }

    for conn in conn_list {
        plot_connection(&mut chart_context, &conn, &kp_list);
    }

    let plot_reaction:bool = false;
    let plot_result_decimals:usize = 0;
    for bc in bc_list {
        plot_boundary_condition(&mut chart_context, &bc, &kp_list, plot_feature_size,plot_reaction,plot_result_decimals);
    }

    for pl in pl_list {
        plot_pointload(&mut chart_context, &pl, &kp_list, plot_feature_size);
    }

    Ok(())
}

pub fn reaction_plot(kp_list:&[Keypoint], 
                     conn_list:&[Connection], 
                     bc_list:&[BoundaryCondition], 
                     pl_list:&[Pointload], 
                     plot_feature_size:f32,
                     output_path:&str,
                     dimension:(u32, u32),
                     chart_title:&str,
                     plot_result_scale:f32,
                     plot_result_decimals:usize,) -> Result<(), Box<dyn std::error::Error>> {

    // Creating the plotting canvas, returning the struct "chart_context"
    let mut chart_context = plot_canvas(&kp_list, &output_path, dimension, &chart_title);

    for conn in conn_list {
        plot_connection(&mut chart_context, &conn, &kp_list);
        plot_connection_displaced(&mut chart_context, &conn, &kp_list, plot_result_scale);
    }

    for kp in kp_list {
        // plot_keypoint(&mut chart_context, &kp, plot_feature_size);
        plot_keypoint_displaced(&mut chart_context, &kp, plot_feature_size, plot_result_scale, plot_result_decimals);
    }

    let plot_reaction:bool = true;
    for bc in bc_list {
        plot_boundary_condition(&mut chart_context, &bc, &kp_list, plot_feature_size,plot_reaction,plot_result_decimals);
    }

    for pl in pl_list {
        plot_pointload(&mut chart_context, &pl, &kp_list, plot_feature_size);
    }
    
    Ok(())
}


