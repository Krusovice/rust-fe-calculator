use crate::input::keypoint::Keypoint;
use crate::input::connection::Connection;
use crate::input::boundary_condition::BoundaryCondition;
use crate::input::pointload::Pointload;

use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;

pub fn plot_keypoint(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
                 keypoint:&Keypoint, plot_feature_size:f32) {
    let x = keypoint.x as f32;
    let y = keypoint.y as f32;
    let _ = chart_context.draw_series(std::iter::once(Circle::new((x,y), plot_feature_size, ShapeStyle::from(&BLACK).filled())));

    let label = format!("{}", keypoint.name);
        plot_label(label, x, y, plot_feature_size, chart_context);
    }

// Plotting resulting keypoint locations, after being displaced.
pub fn plot_keypoint_displaced(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
                 keypoint:&Keypoint, plot_feature_size:f32, plot_result_scale:f32, plot_result_decimals:usize) {
    let x = keypoint.x as f32 + keypoint.ux as f32 * plot_result_scale;
    let y = keypoint.y as f32 + keypoint.uy as f32 * plot_result_scale;
    let _ = chart_context.draw_series(std::iter::once(Circle::new((x,y), plot_feature_size, ShapeStyle::from(&RED).filled())));
    
    if keypoint.ux != 0.0 || keypoint.uy != 0.0 {
        let label = format!("U ({:.2$}, {:.2$})", keypoint.ux, keypoint.uy, plot_result_decimals);
        plot_label(label, x, y, plot_feature_size, chart_context);
        }
    }

pub fn plot_connection(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
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

pub fn plot_connection_displaced(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
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

pub fn plot_boundary_condition(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
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
            let pixel_offset_y:i32 = (plot_feature_size*6.0) as i32;

            // 
            let (x_new, y_new) = offset_label_coordinates(chart_context,x,y,0,pixel_offset_y);

            let label = format!("F ({:.2$}, {:.2$})", kp.fx, kp.fy, plot_result_decimals);
            plot_label(label, x_new, y_new, plot_feature_size, chart_context);
        }
    }
}

pub fn plot_pointload(chart_context:&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>, 
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

pub fn plot_label(text_label: String, x:f32, y:f32, plot_feature_size: f32,
              chart_context: &mut ChartContext<BitMapBackend, 
                                  Cartesian2d<RangedCoordf32, 
                                  RangedCoordf32>>) {
    let text_size: i32 = (plot_feature_size*6.0) as i32;
    let label = Text::new(text_label, (x+plot_feature_size/50.0, y+plot_feature_size/100.0),("sans-serif", text_size).into_font().color(&BLACK));
    let _ = chart_context.draw_series(std::iter::once(label));
    }

// Function that takes in x and y in canvas units, and offset coordinates x and y in pixels.
// Returns the offset coordinates in canvas units.
fn offset_label_coordinates(chart_context: &ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
                               x:f32, y:f32, offset_pixels_x:i32, offset_pixels_y:i32) -> (f32, f32) {

    let x_range = chart_context.as_coord_spec().x_spec();
    let y_range = chart_context.as_coord_spec().y_spec();
    let canvas_x_range = x_range.range().end - x_range.range().start;
    let canvas_y_range = y_range.range().end - y_range.range().start;

    // Finding amount of pixels in canvas.
    let canvas_pixels = chart_context.plotting_area().get_pixel_range();
    let (x_pixels_range, y_pixels_range) = canvas_pixels;
    let pixels_x = (x_pixels_range.end - x_pixels_range.start) as f32;
    let pixels_y = (y_pixels_range.end - y_pixels_range.start) as f32;

    // Defining units per pixel.
    let units_per_pixel_x = canvas_x_range / pixels_x;
    let units_per_pixel_y = canvas_y_range / pixels_y;

    // Setting unit offset based on pixels.
    let units_offset_x = (units_per_pixel_x * offset_pixels_x as f32);
    let units_offset_y = (units_per_pixel_y * offset_pixels_y as f32);

    // Setting the new y coordinate
    let x_offset = x as f32 + units_offset_x;
    let y_offset = y as f32 + units_offset_y;

    (x_offset, y_offset)
}