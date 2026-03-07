use flo_curves::*;
use flo_curves::bezier;
use flo_curves::bezier::{NormalCurve};
use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;
use flo_draw::*;
use flo_draw::canvas::*;

use flo_curves::geo::{Coord2};

use std::f64;
use std::thread;
use std::time::{Duration};

///
/// Draws a thick bezier path using the 'stroke' algorithm
///
fn show_stroke_path<TPath>(gc: &mut impl GraphicsContext, path: TPath, options: StrokeOptions) 
where
    TPath:          BezierPath + BezierPathFactory,
    TPath::Point:   Coordinate + Coordinate2D,
{
    let stroked_path = stroke_path::<TPath, _>(&path, 32.0, &options);

    gc.new_path();
    gc.line_width(1.0);
    gc.stroke_color(Color::Rgba(0.7, 0.2, 0.0, 1.0));
    gc.bezier_path(&path);
    gc.stroke();

    gc.new_path();
    gc.line_width(1.0);
    gc.stroke_color(Color::Rgba(0.0, 0.2, 0.7, 1.0));
    for path in stroked_path {
        gc.bezier_path(&path);
    }
    gc.stroke();
}

fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("Stroke demo");

        canvas.draw(|gc| {
            // Clear the canvas
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            // Set up a curve path
            let sample_path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(100.0, 100.0))
                .curve_to((Coord2(120.0, 150.0), Coord2(170.0, 50.0)), Coord2(200.0, 100.0))
                .curve_to((Coord2(220.0, 150.0), Coord2(280.0, 50.0)), Coord2(300.0, 100.0))
                .build();
            let options = StrokeOptions::default()
                .with_start_cap(flo_curves::bezier::path::LineCap::Square)
                .with_end_cap(flo_curves::bezier::path::LineCap::Square);
            show_stroke_path(gc, sample_path, options);

            let sample_path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(100.0, 200.0))
                .curve_to((Coord2(120.0, 250.0), Coord2(170.0, 150.0)), Coord2(200.0, 200.0))
                .curve_to((Coord2(220.0, 250.0), Coord2(280.0, 150.0)), Coord2(300.0, 200.0))
                .build();
            let options = StrokeOptions::default()
                .with_start_cap(flo_curves::bezier::path::LineCap::Round)
                .with_end_cap(flo_curves::bezier::path::LineCap::Round);
            show_stroke_path(gc, sample_path, options);
        })
    });
}
