use crate::input::keypoint::Keypoint;
use crate::input::connection::Connection;
use crate::input::boundary_condition::BoundaryCondition;
use crate::input::pointload::Pointload;

use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;


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

    for kp in kp_list {
        plot_keypoint(&mut chart_context, &kp, plot_feature_size);
        plot_keypoint_displaced(&mut chart_context, &kp, plot_feature_size, plot_result_scale, plot_result_decimals);
    }

    for conn in conn_list {
        plot_connection(&mut chart_context, &conn, &kp_list);
        plot_connection_displaced(&mut chart_context, &conn, &kp_list, plot_result_scale);
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

fn plot_canvas<'a>(kp_list:&[Keypoint],
                   output_path:&'a str,
                   dimension: (u32, u32),
                   chart_title:&str) -> ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf32, RangedCoordf32>> {
    
    // Setting an equal x and y scale.      
    let min_x:f32 = kp_list.iter().map(|kp| kp.x as f32).fold(f32::INFINITY, |acc, x| f32::min(acc,x));
    let max_x:f32 = kp_list.iter().map(|kp| kp.x as f32).fold(f32::NEG_INFINITY, |acc, x| f32::max(acc,x));
    let min_y:f32 = kp_list.iter().map(|kp| kp.y as f32).fold(f32::INFINITY, |acc, y| f32::min(acc,y));
    let max_y:f32 = kp_list.iter().map(|kp| kp.y as f32).fold(f32::NEG_INFINITY, |acc, y| f32::max(acc,y));

    // Defining pixels for calculating scale
    let (width_px, height_px) = dimension;
    
    // Finding the ranges
    let x_range = max_x - min_x;
    let y_range = max_y - min_y;

    // Finding the scale for x and y
    let x_scale = x_range / width_px as f32;
    let y_scale = y_range / height_px as f32;

    // Use the larger scale to make both axes "equal"
    let uniform_scale = x_scale.max(y_scale);

    // Finding the chart center
    let x_center = (min_x + max_x) / 2.0;
    let y_center = (min_y + max_y) / 2.0;


    let half_width = (uniform_scale * width_px as f32) / 2.0;
    let half_height = (uniform_scale * height_px as f32) / 2.0;

    let min_x = x_center - half_width;
    let max_x = x_center + half_width;
    let min_y = y_center - half_height;
    let max_y = y_center + half_height;

    let backend = BitMapBackend::new(output_path, dimension);
    let drawing_area = backend.into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    // Creating chart
    let mut chart_context = ChartBuilder::on(&drawing_area)
        // Set the caption of the chart
        .caption(chart_title, ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(min_x-1.0..max_x+1.0, min_y-1.0..max_y+1.0)
        .unwrap();

    // Drawing background mesh
    chart_context
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        .draw()
        .unwrap();
    drawing_area.present().unwrap();
    chart_context
}

fn plot_keypoint(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
                 keypoint:&Keypoint, plot_feature_size:f32) {
    let x = keypoint.x as f32;
    let y = keypoint.y as f32;
    let _ = chart_context.draw_series(std::iter::once(Circle::new((x,y), plot_feature_size, ShapeStyle::from(&BLACK).filled())));

    let label = format!("{}", keypoint.name);
        plot_label(label, x, y, plot_feature_size, chart_context);
    }

// Plotting resulting keypoint locations, after being displaced.
fn plot_keypoint_displaced(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
                 keypoint:&Keypoint, plot_feature_size:f32, plot_result_scale:f32, plot_result_decimals:usize) {
    let x = keypoint.x as f32 + keypoint.ux as f32 * plot_result_scale;
    let y = keypoint.y as f32 + keypoint.uy as f32 * plot_result_scale;
    let _ = chart_context.draw_series(std::iter::once(Circle::new((x,y), plot_feature_size, ShapeStyle::from(&RED).filled())));
    
    if keypoint.ux != 0.0 || keypoint.uy != 0.0 {
        let label = format!("U ({:.2$}, {:.2$})", keypoint.ux, keypoint.uy, plot_result_decimals);
        plot_label(label, x, y, plot_feature_size, chart_context);
        }
    }

fn plot_connection(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
                 connection:&Connection, kp_list:&[Keypoint]) {

    // Finding the keypoint coordinates through the keypoint struct
    let kp1 = kp_list.iter().find(|kp| kp.name == connection.kp_1).unwrap();
    let kp2 = kp_list.iter().find(|kp| kp.name == connection.kp_2).unwrap();
    let kp1_x = kp1.x as f32;
    let kp1_y = kp1.y as f32;
    let kp2_x = kp2.x as f32;
    let kp2_y = kp2.y as f32;

    let _ = chart_context.draw_series(LineSeries::new(vec![(kp1_x, kp1_y), (kp2_x, kp2_y)],&BLACK));
    }

fn plot_connection_displaced(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
                 connection:&Connection, kp_list:&[Keypoint], plot_result_scale:f32) {

    // Finding the keypoint coordinates through the keypoint struct
    let kp1 = kp_list.iter().find(|kp| kp.name == connection.kp_1).unwrap();
    let kp2 = kp_list.iter().find(|kp| kp.name == connection.kp_2).unwrap();
    let kp1_x = kp1.x as f32 + kp1.ux as f32 * plot_result_scale;
    let kp1_y = kp1.y as f32 + kp1.uy as f32 * plot_result_scale;
    let kp2_x = kp2.x as f32 + kp2.ux as f32 * plot_result_scale;
    let kp2_y = kp2.y as f32 + kp2.uy as f32 * plot_result_scale;

    let _ = chart_context.draw_series(LineSeries::new(vec![(kp1_x, kp1_y), (kp2_x, kp2_y)],&RED));
    }    

fn plot_boundary_condition(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
                 boundary_condition:&BoundaryCondition, kp_list:&[Keypoint], plot_feature_size:f32,plot_reaction:bool, plot_result_decimals:usize) {

    let size:f32 = plot_feature_size/15.0;

    // Finding the keypoint of the boundary condition, and its x and y coordinates.
    let kp = kp_list.iter().find(|kp| kp.name == boundary_condition.keypoint).unwrap();
    let x = kp.x as f32;
    let y = kp.y as f32;

    // Drawing the triangle that defines the boundary condition. Based on the fixture direction.
    // 0 = vertically fixed bc
    // 1 = laterally fixed bc
    // 2 = both vertically and laterally fixed bc
    if boundary_condition.fixture == "0" {
        let triangle = PathElement::new(vec![(x, y),(x-size/2.0, y-size),(x+size/2.0, y-size),(x, y)],ShapeStyle::from(&BLACK).filled());
        chart_context.draw_series(std::iter::once(triangle)).unwrap();
        let _ = chart_context.draw_series(LineSeries::new(vec![(x-size/2.0, y-size*1.2), (x+size/2.0, y-size*1.2)],&BLACK));

    } else if boundary_condition.fixture == "1" {
        let triangle = PathElement::new(vec![(x, y),(x-size, y-size/2.0),(x-size, y+size/2.0),(x, y)],ShapeStyle::from(&BLACK).filled());
        chart_context.draw_series(std::iter::once(triangle)).unwrap();
        let _ = chart_context.draw_series(LineSeries::new(vec![(x-size*1.2, y-size/2.0), (x-size*1.2, y+size/2.0)],&BLACK));

    } else if boundary_condition.fixture == "2" {
        let triangle = PathElement::new(vec![(x, y),(x-size/2.0, y-size),(x+size/2.0, y-size),(x, y)],ShapeStyle::from(&BLACK).filled());
        chart_context.draw_series(std::iter::once(triangle)).unwrap();
    }

    if plot_reaction {
        if kp.fx != 0.0 || kp.fy != 0.0 {
        let label = format!("F ({:.2$}, {:.2$})", kp.fx, kp.fy, plot_result_decimals);
        plot_label(label, x, y, plot_feature_size, chart_context);
        }
    }
}

fn plot_pointload(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
                 pointload:&Pointload, kp_list:&[Keypoint], plot_feature_size:f32) {
    let size:f32 = plot_feature_size/7.0;

    // Finding the keypoint coordinates through the keypoint struct
    let kp = kp_list.iter().find(|kp| kp.name == pointload.keypoint).unwrap();
    let x = kp.x as f32;
    let y = kp.y as f32;

    // Finding the force direction, and defining x and y directions between 0 and 1.
    let max_force: f32 = (pointload.load_x.abs() as f32).max(pointload.load_y.abs() as f32);
    let x_direction:f32 = pointload.load_x as f32 / max_force;
    let y_direction:f32 = pointload.load_y as f32 / max_force;
    
    // Drawing pointload
    let arrow_straight_line = LineSeries::new(vec![(x, y), (x-x_direction*size, y-y_direction*size)],&BLACK);
    let _ = chart_context.draw_series(arrow_straight_line);
    let triangle = PathElement::new(vec![(x, y),
                                         (x+(y_direction-x_direction)*size/3.0, y-(x_direction+y_direction)*size/3.0),
                                         (x-(y_direction+x_direction)*size/3.0, y+(x_direction-y_direction)*size/3.0),
                                         (x, y)],ShapeStyle::from(&BLACK).filled());
    chart_context.draw_series(std::iter::once(triangle)).unwrap();
    
    //plot_label(format!("PL ({}, {})", pointload.load_x, pointload.load_y),
    //           x, y, plot_feature_size, chart_context);
    }

fn plot_label(text_label: String, x:f32, y:f32, plot_feature_size: f32,
              chart_context: &mut ChartContext<BitMapBackend, 
                                  Cartesian2d<RangedCoordf32, 
                                  RangedCoordf32>>) {
    let text_size: i32 = (plot_feature_size*6.0) as i32;
    let label = Text::new(text_label, (x+plot_feature_size/50.0, y+plot_feature_size/100.0),("sans-serif", text_size).into_font().color(&BLACK));
    let _ = chart_context.draw_series(std::iter::once(label));
    }