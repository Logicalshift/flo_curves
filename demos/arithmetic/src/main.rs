use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;
use flo_draw::*;
use flo_draw::canvas::*;

use std::f64;
use std::thread;
use std::time::{Duration, Instant};

fn path_convolution(path: Vec<Coord2>, start_offset: usize, forward: bool) -> SimpleBezierPath {
    let mut result = BezierPathBuilder::start(path[start_offset]);

    for idx in 1..path.len() {
        let pos = if forward {
            (start_offset + idx) % path.len()
        } else {
            let idx = (path.len()) - idx;
            (start_offset + idx) % path.len()
        };

        result = result.line_to(path[pos]);
   }

    result.build()
}

fn main() {
    with_2d_graphics(|| {
        let canvas          = create_canvas_window("Path arithmetic demonstration");

        let start_time = Instant::now();

        loop {
            // Wait for the next frame
            thread::sleep(Duration::from_nanos(1_000_000_000 / 60));

            // Decide on an amplitude that determines where the paths are relative to each other
            let since_start     = Instant::now().duration_since(start_time);
            let since_start     = since_start.as_nanos() as f64;
            let amplitude       = (since_start / (f64::consts::PI * 1_000_000_000.0)).cos() * 200.0;

            // Create some circles
            let path1           = Circle::new(Coord2(500.0 + amplitude, 500.0), 100.0).to_path::<SimpleBezierPath>();
            let path2           = Circle::new(Coord2(500.0 - amplitude, 500.0), 100.0).to_path::<SimpleBezierPath>();
            let path3           = Circle::new(Coord2(500.0, 500.0 - amplitude), 60.0).to_path::<SimpleBezierPath>();

            // Test path
            let path4           = path_convolution(vec![Coord2(206.0, 391.0), Coord2(206.0, 63.0), Coord2(281.0, 66.0), Coord2(281.0, 320.0), Coord2(649.0, 320.0), Coord2(649.0, 63.0), Coord2(734.0, 63.0), Coord2(734.0, 391.0)], 0, true);
            let path5           = path_convolution(vec![Coord2(64.0, 263.0), Coord2(877.0, 263.0), Coord2(877.0, 168.0), Coord2(64.0, 168.0)], 0, false);

            // Add and subtract them to generate the final path
            let path            = path_add::<_, _, SimpleBezierPath>(&vec![path1.clone()], &vec![path2.clone()], 0.1);
            let path            = path_sub::<_, _, SimpleBezierPath>(&path, &vec![path3.clone()], 0.1);

            let sub_path_test   = path_sub::<_, _, SimpleBezierPath>(&vec![path5], &vec![path4], 0.1);

            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));

                gc.canvas_height(1000.0);
                gc.center_region(0.0, 0.0, 1000.0, 1000.0);

                // Render the subpaths
                gc.line_width(1.0);

                gc.stroke_color(Color::Rgba(0.4, 0.8, 0.0, 0.5));
                for p in vec![&path1, &path2, &path3] {
                    gc.new_path();
                    gc.bezier_path(p);
                    gc.stroke();
                }

                // Render the combined path
                gc.line_width(3.0);

                gc.stroke_color(Color::Rgba(0.8, 0.5, 0.0, 1.0));
                gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 0.8));
                gc.winding_rule(WindingRule::EvenOdd);

                gc.new_path();
                path.iter().for_each(|path| {
                    gc.bezier_path(path);
                });
                gc.fill();
                gc.stroke();

                gc.new_path();
                sub_path_test.iter().for_each(|sub_path_test| {
                    gc.bezier_path(sub_path_test);
                });
                gc.fill();
                gc.stroke();

                // Render the path points
                gc.line_width(1.0);

                for subpath in path.iter() {
                    for (_, _, point) in subpath.1.iter() {
                        gc.new_path();
                        gc.circle(point.x() as _, point.y() as _, 5.0);
                        gc.stroke();
                    }
                }
            });
        }
    });
}
