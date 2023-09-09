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
            let offset_curve_2  = bezier::offset_lms_subdivisions(&initial_curve, |t| -((off2-off1)*t+off1), |_| 0.0, &bezier::SubdivisionOffsetOptions::default()).unwrap();
            //let offset_curve_3  = bezier::offset_lms_sampling(&initial_curve, |t| ((off2-off1)*t+off1) * (t*32.0).cos(), |_| 0.0, 200, 1.0).unwrap();

            let curve_path      = SimpleBezierPath::from_points(p0, vec![(p1, p2, p3)]);
            let stroked_curve   = stroke_path::<SimpleBezierPath, _>(&curve_path, 10.0, &StrokeOptions::default().with_accuracy(0.25).with_remove_interior_points());

            // Create a curve with radius to use with the brush stroke algorithm
            let p0_3 = Coord3::from((p0, off1));
            let p1_3 = Coord3::from((p1, (off2-off1)*(1.0/3.0) + off1));
            let p2_3 = Coord3::from((p2, (off2-off1)*(2.0/3.0) + off1));
            let p3_3 = Coord3::from((p3, off2));

            // Create a distance field for the brush stroke
            let brush_curve     = bezier::Curve::from_points(p0_3, (p1_3, p2_3), p3_3);

            // Render to a path
            let offset_curve_3  = brush_stroke_from_curve::<SimpleBezierPath, _, _>(&CircularBrush, &brush_curve, 0.5, 0.25);

            let mut has_weird_curve = false;
            for path in offset_curve_3.iter() {
                for curve in path.to_curves::<bezier::Curve<Coord2>>() {
                    let (sp, (cp1, cp2), ep) = curve.all_points();
                    let (d1, d2, d3) = (sp.distance_to(&cp1), cp2.distance_to(&ep), sp.distance_to(&ep));

                    if (d1 > d3 * 10.0) || (d2 > d3 * 10.0) {
                        has_weird_curve = true;
                        break;
                    }
                }
            }

            if has_weird_curve {
                println!("Problem? {:?} {:?} {:?} {:?}", counter, pos, off1, off2);
            }

            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));

                gc.canvas_height(1000.0);
                gc.center_region(0.0, 0.0, 1000.0, 1000.0);
                
                gc.line_width(2.0);

                gc.new_path();
                for path in stroked_curve.iter() {
                    gc.bezier_path(path);
                }
                gc.fill_color(Color::Rgba(0.0, 0.5, 0.0, 1.0));
                gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
                gc.fill();
                gc.stroke();

                gc.new_path();
                gc.move_to(initial_curve.start_point().x() as _, initial_curve.start_point().y() as _);
                let (cp1, cp2)  = initial_curve.control_points();
                let end         = initial_curve.end_point();

                gc.bezier_curve_to(end.x() as _, end.y() as _, cp1.x() as _, cp1.y() as _, cp2.x() as _, cp2.y() as _);
                gc.stroke_color(Color::Rgba(0.0, 0.0, 1.0, 1.0));
                gc.stroke();

                gc.new_path();
                gc.move_to(offset_curve_1[0].start_point().x() as _, offset_curve_1[0].start_point().y() as _);
                for c in offset_curve_1.iter() {
                    let (cp1, cp2)  = c.control_points();
                    let end         = c.end_point();

                    gc.bezier_curve_to(end.x() as _, end.y() as _, cp1.x() as _, cp1.y() as _, cp2.x() as _, cp2.y() as _);
                }
                gc.stroke_color(Color::Rgba(1.0, 0.0, 0.0, 0.5));
                gc.stroke();

                gc.new_path();
                gc.move_to(offset_curve_2[0].start_point().x() as _, offset_curve_2[0].start_point().y() as _);
                for c in offset_curve_2.iter() {
                    let (cp1, cp2)  = c.control_points();
                    let end         = c.end_point();

                    gc.bezier_curve_to(end.x() as _, end.y() as _, cp1.x() as _, cp1.y() as _, cp2.x() as _, cp2.y() as _);
                }
                gc.stroke_color(Color::Rgba(0.0, 0.6, 0.0, 0.5));
                gc.stroke();

                gc.new_path();
                for subpath in offset_curve_3.iter() {
                    let curve   = subpath.to_curves::<bezier::Curve<Coord2>>();

                    gc.move_to(curve[0].start_point().x() as _, curve[0].start_point().y() as _);
                    for c in curve.iter() {
                        let (cp1, cp2)  = c.control_points();
                        let end         = c.end_point();

                        gc.bezier_curve_to(end.x() as _, end.y() as _, cp1.x() as _, cp1.y() as _, cp2.x() as _, cp2.y() as _);
                    }
                }
                gc.line_width(3.0);
                gc.stroke_color(Color::Rgba(0.6, 0.3, 0.0, 1.0));
                gc.stroke();

                gc.line_width(1.0);
                gc.stroke_color(Color::Rgba(0.0, 0.6, 0.4, 0.25));

                for circle_t in 0..=20 {
                    let circle_t        = (circle_t as f64) / 20.0;
                    let center          = initial_curve.point_at_pos(circle_t);
                    let normal          = initial_curve.normal_at_pos(circle_t).to_unit_vector();
                    let radius          = (off2-off1)*circle_t+off1;
                    let normal_offset   = center + (normal * radius);

                    gc.new_path();
                    gc.circle(center.x() as _, center.y() as _, radius as _);
                    gc.move_to(center.x() as _, center.y() as _);
                    gc.line_to(normal_offset.x() as _, normal_offset.y() as _);
                    gc.stroke();
                }

                for curve in vec![&vec![initial_curve], &offset_curve_1, &offset_curve_2].into_iter() {
                    gc.line_width(1.0);
                    gc.stroke_color(Color::Rgba(0.6, 0.0, 0.0, 0.25));
                    if let Some(c) = curve.iter().next() {
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
