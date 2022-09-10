use flo_curves::*;
use flo_curves::bezier;
use flo_draw::*;
use flo_draw::canvas::*;

use std::f64;
use std::thread;
use std::time::{Duration};

fn main() {
    with_2d_graphics(|| {
        let canvas          = create_canvas_window("Offset demo");
        let mut counter     = 0;

        loop {
            thread::sleep(Duration::from_nanos(1_000_000_000 / 60));

            counter             = counter + 1;

            let pos             = (counter as f64)/400.0 * 2.0*f64::consts::PI;
            let pos             = (pos.sin() + 1.0) * 200.0;
            let off1            = 200.0 - pos/2.0;
            let off2            = pos/2.0;

            // Borrowing the animation from https://www.shadertoy.com/view/4sKyzW because it's interesting (and because I'm interested in using distance fields to make an improved offset algorithm)
            let t  = (counter as f64) / 40.0; 
            let p0 = Coord2(-(t*1.0/2.0).cos() * 400.0, (t*1.0/3.0).sin() * 500.0) + Coord2(500.0, 500.0);
            let p1 = Coord2(-(t*2.0/3.0).cos() * 400.0, (t*1.0/4.0).sin() * 200.0) + Coord2(500.0, 500.0);
            let p2 = Coord2((t*1.0/4.0).cos() * 200.0, -(t*2.0/3.0).sin() * 400.0) + Coord2(500.0, 500.0);
            let p3 = Coord2((t*1.0/3.0).cos() * 500.0, -(t*1.0/2.0).sin() * 200.0) + Coord2(500.0, 500.0);

            let initial_curve   = bezier::Curve::from_points(p0, (p1, p2), p3);
            let offset_curve_1  = bezier::offset(&initial_curve, off1, off2);
            let offset_curve_2  = bezier::offset_lms_sampling(&initial_curve, |t| -((off2-off1)*t+off1), |_| 0.0, 40, 1.0).unwrap();
            let offset_curve_3  = bezier::offset_lms_sampling(&initial_curve, |t| ((off2-off1)*t+off1) * (t*32.0).cos(), |_| 0.0, 200, 1.0).unwrap();

            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));

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

                gc.new_path();
                gc.move_to(offset_curve_3[0].start_point().x() as _, offset_curve_3[0].start_point().y() as _);
                for c in offset_curve_3.iter() {
                    gc.bezier_curve(c);
                }
                gc.stroke_color(Color::Rgba(0.0, 0.6, 0.4, 0.25));
                gc.stroke();

                for curve in vec![&vec![initial_curve], &offset_curve_1, &offset_curve_2, &offset_curve_3].into_iter() {
                    gc.line_width(1.0);
                    gc.stroke_color(Color::Rgba(0.6, 0.0, 0.0, 0.25));
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
            });
        }
    });
}
