use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_draw::*;
use flo_draw::canvas::*;

fn main() {
    with_2d_graphics(|| {
        let canvas          = create_canvas_window("Curve intersection demonstration");

        // Two paths to find intersections in
        let path1 = Circle::new(Coord2(496.9997044935593, 500.0), 300.0).to_path::<SimpleBezierPath>();
        let path2 = Circle::new(Coord2(503.0002955064407, 500.0), 300.0).to_path::<SimpleBezierPath>();

        // Find the intersections between these two paths
        let mut intersections = vec![];
        for curve1 in path1.to_curves::<Curve<_>>() {
            for curve2 in path2.to_curves::<Curve<_>>() {
                intersections.extend(curve_intersects_curve_clip(&curve1, &curve2, 0.01).into_iter()
                    .map(|(t1, _t2)| curve1.point_at_pos(t1)));
            }
        }

        println!("{:?}", intersections.len());

        // Draw the circles and the intersections
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));

            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            // Render the subpaths
            gc.line_width(1.0);

            gc.stroke_color(Color::Rgba(0.4, 0.8, 0.0, 1.0));
            for p in vec![&path1, &path2] {
                gc.new_path();
                gc.bezier_path(p);
                gc.stroke();
            }

            // Render the intersection points
            gc.stroke_color(Color::Rgba(0.8, 0.4, 0.0, 1.0));

            for Coord2(x, y) in intersections {
                gc.new_path();
                gc.circle(x as _, y as _, 5.0);
                gc.stroke();
            }
        })
    });
}
