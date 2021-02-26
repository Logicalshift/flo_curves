use flo_curves::*;
use flo_curves::bezier;
use flo_draw::*;
use flo_draw::canvas::*;

use std::time::{Instant,Duration};

fn main() {
    with_2d_graphics(|| {
        let canvas          = create_canvas_window("Offset demo");
        let initial_curve   = bezier::Curve::from_points(Coord2(100.0, 100.0), (Coord2(200.0, 800.0), Coord2(1000.5, 1000.0)), Coord2(900.0, 700.0));
        let offset_curve_1  = bezier::offset(&initial_curve, -200.0, -5.0);
        let offset_curve_2  = bezier::offset_lms_sampling(&initial_curve, |t| -((5.0-200.0)*t+200.0), 20, 4.0).unwrap();

        let time_1 = Instant::now();
        let mut count = 0;
        while time_1.elapsed() < Duration::from_millis(500) {
            count += 1;
            bezier::offset(&initial_curve, 80.0, 5.0);
        }
        println!("Scaling: {:?}/sec", count *2);

        let time_2 = Instant::now();
        let mut count = 0;
        while time_2.elapsed() < Duration::from_millis(500) {
            count += 1;
            bezier::offset_lms_sampling(&initial_curve, |t| -((5.0-80.0)*t+80.0), 20, 0.5).unwrap();
        }
        println!("LMS: {:?}/sec", count *2);

        canvas.draw(|gc| {
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);
            
            gc.line_width(2.0);

            gc.new_path();
            gc.move_to(initial_curve.start_point().x() as _, initial_curve.start_point().y() as _);
            gc.bezier_curve(&initial_curve);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 1.0, 1.0));
            gc.stroke();

            gc.new_path();
            gc.move_to(offset_curve_1[0].start_point().x() as _, offset_curve_1[0].start_point().y() as _);
            for c in offset_curve_1.iter() {
                gc.bezier_curve(c);
            }
            gc.stroke_color(Color::Rgba(1.0, 0.0, 0.0, 1.0));
            gc.stroke();

            gc.new_path();
            gc.move_to(offset_curve_2[0].start_point().x() as _, offset_curve_2[0].start_point().y() as _);
            for c in offset_curve_2.iter() {
                gc.bezier_curve(c);
            }
            gc.stroke_color(Color::Rgba(0.0, 0.6, 0.0, 1.0));
            gc.stroke();

            for curve in vec![&vec![initial_curve], &offset_curve_1, &offset_curve_2].into_iter() {
                gc.line_width(1.0);
                gc.stroke_color(Color::Rgba(0.6, 0.0, 0.0, 1.0));
                for c in curve.iter().nth(0) {
                    let p = c.start_point();
                    gc.new_path();
                    gc.circle(p.x() as _, p.y() as _, 6.0);
                    gc.stroke();
                }
                for c in curve.iter() {
                    let p = c.end_point();
                    gc.new_path();
                    gc.circle(p.x() as _, p.y() as _, 6.0);
                    gc.stroke();
                }
            }
        })
    });
}
