use crate::input::keypoint::Keypoint;
use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;

pub fn plot_canvas<'a>(kp_list:&[Keypoint],
                   output_path:&'a str,
                   dimension: (u32, u32),
                   chart_title:&str) -> ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf32, RangedCoordf32>> {
    
    // Setting an equal x and y scale.
    // Finding the min and max value for the keypoints. Adding +/- 1 to the values for plot edges.
    let min_x:f32 = kp_list.iter().map(|kp| kp.x as f32).fold(f32::INFINITY, |acc, x| f32::min(acc,x)) - 1.0 ;
    let max_x:f32 = kp_list.iter().map(|kp| kp.x as f32).fold(f32::NEG_INFINITY, |acc, x| f32::max(acc,x)) +1.0 ;
    let min_y:f32 = kp_list.iter().map(|kp| kp.y as f32).fold(f32::INFINITY, |acc, y| f32::min(acc,y)) -1.0 ;
    let max_y:f32 = kp_list.iter().map(|kp| kp.y as f32).fold(f32::NEG_INFINITY, |acc, y| f32::max(acc,y)) +1.0;

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