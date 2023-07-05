use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;
use flo_draw::*;
use flo_draw::canvas::*;

use flo_curves::geo::{Coord2};

use std::f64;

///
/// Draws the outline of a path
///
fn draw_path_outline(gc: &mut (impl GraphicsPrimitives + GraphicsContext), path: impl IntoIterator<Item=SimpleBezierPath>, col1: Color, col2: Color) {
    gc.new_path();

    for subpath in path {
        let sp = subpath.start_point();
        gc.move_to(sp.x() as _, sp.y() as _);

        for curve in subpath.to_curves::<Curve<_>>() {
            let (_, (cp1, cp2), ep) = curve.all_points();
            gc.bezier_curve_to(ep.x() as _, ep.y() as _, cp1.x() as _, cp1.y() as _, cp2.x() as _, cp2.y() as _);
        }
    }

    // Thick 'outer' path
    gc.line_width(4.0);
    gc.stroke_color(col1);
    gc.stroke();

    // Thin 'inner' path
    gc.line_width(2.0);
    gc.stroke_color(col2);
    gc.stroke();
}

///
/// Draws the outline of a simple brush stroke using the 'circular' brush head
///
fn draw_circle_brush_stroke(gc: &mut (impl GraphicsPrimitives + GraphicsContext), center_x: f64, length: f64) {
    // Create some curves by fitting along the length
    let brush_stroke = (0..(length as isize))
        .map(|p| {
            // p gives us the y position
            let p       = p as f64;
            let y_pos   = p + 100.0;

            let p = p / 800.0;
            let p = p * f64::consts::PI;

            let x_pos = center_x + (p*7.0).sin()*32.0;
            let width = p.sin().abs() * 10.0;

            Coord3(x_pos, y_pos, width)
        });
    let brush_stroke = fit_curve::<Curve<_>>(&brush_stroke.collect::<Vec<_>>(), 0.1).unwrap();
    let brush_stroke = SimpleBezierPath3::from_connected_curves(brush_stroke);

    // Use the circular brush
    let brush       = CircularBrush;

    // Use the brush to create a brush stroke path
    let brush_stroke_path = brush_stroke_from_path::<SimpleBezierPath, _, _>(&brush, &brush_stroke, 0.5, 1.0);

    // Draw it as a preview
    draw_path_outline(gc, brush_stroke_path, Color::Rgba(1.0, 0.8, 0.8, 1.0), Color::Rgba(0.1, 0.1, 0.1, 1.0));
}

///
/// Draws the outline of a simple brush stroke alongside an image of the brush head
///
fn draw_path_brush_stroke(gc: &mut (impl GraphicsPrimitives + GraphicsContext), center_x: f64, length: f64, brush_head: Vec<SimpleBezierPath>) {
    let bounds = brush_head.iter().map(|subpath| subpath.bounding_box::<Bounds<_>>()).reduce(|a, b| a.union_bounds(b)).unwrap();

    let length = length.max(0.0);
    let length = length.min(800.0);

    // Draw the brush preview
    let offset  = bounds.min();
    let size    = bounds.max() - bounds.min();
    let scale   = size.x().max(size.y());

    let preview = brush_head.iter()
        .map(|subpath| {
            subpath.map_points::<SimpleBezierPath>(|point| {
                (point - offset - (size*0.5)) * (1.0/scale) * 32.0 + Coord2(center_x, 50.0)
            })
        });

    draw_path_outline(gc, preview, Color::Rgba(0.4, 0.85, 1.0, 1.0), Color::Rgba(0.1, 0.1, 0.1, 1.0));

    // Create some curves by fitting along the length
    let brush_stroke = (0..(length as isize))
        .map(|p| {
            // p gives us the y position
            let p       = p as f64;
            let y_pos   = p + 100.0;

            let p = p / 800.0;
            let p = p * f64::consts::PI;

            let x_pos = center_x + (p*7.0).sin()*32.0;
            let width = p.sin().abs() * 20.0;

            Coord3(x_pos, y_pos, width)
        });
    let brush_stroke = fit_curve::<Curve<_>>(&brush_stroke.collect::<Vec<_>>(), 0.1).unwrap();
    let brush_stroke = SimpleBezierPath3::from_connected_curves(brush_stroke);

    // Create a brush from the path
    let (field, _)  = PathDistanceField::center_path(brush_head);
    let brush       = ScaledBrush::from_distance_field(&field);
    let brush       = &brush;

    // Use the brush to create a brush stroke path
    let brush_stroke_path = brush_stroke_from_path::<SimpleBezierPath, _, _>(&brush, &brush_stroke, 0.5, 1.0);

    // Draw it as a preview
    draw_path_outline(gc, brush_stroke_path, Color::Rgba(1.0, 0.8, 0.8, 1.0), Color::Rgba(0.1, 0.1, 0.1, 1.0));
}

fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("Brush demo");

        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));

            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
                .line_to(Coord2(12.0, 36.0))
                .line_to(Coord2(36.0, 48.0))
                .line_to(Coord2(24.0, 12.0))
                .line_to(Coord2(0.0, 0.0))
                .build();

            draw_circle_brush_stroke(gc, 100.0, 800.0);
            draw_path_brush_stroke(gc, 200.0, 800.0, vec![Circle::new(Coord2(0.0, 0.0), 32.0).to_path::<SimpleBezierPath>()]);
            draw_path_brush_stroke(gc, 300.0, 800.0, vec![chisel]);
        });
    });
}
