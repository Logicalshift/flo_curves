use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;
use flo_draw::*;
use flo_draw::canvas::*;

use std::f64;
use std::thread;
use std::time::{Duration, Instant};

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
            let path2           = Circle::new(Coord2(500.0 - amplitude, 500.0), 120.0).to_path::<SimpleBezierPath>();
            let path3           = Circle::new(Coord2(500.0, 500.0 - amplitude), 60.0).to_path::<SimpleBezierPath>();

            // Add and subtract them to generate the final path
            let path            = path_add::<_, _, SimpleBezierPath>(&vec![path1], &vec![path2], 0.1);
            let path            = path_sub::<_, _, SimpleBezierPath>(&path, &vec![path3], 0.1);

            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));

                gc.canvas_height(1000.0);
                gc.center_region(0.0, 0.0, 1000.0, 1000.0);
                
                gc.line_width(3.0);

                // Render the combined path
                gc.stroke_color(Color::Rgba(0.8, 0.5, 0.0, 1.0));
                gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 0.8));
                gc.winding_rule(WindingRule::EvenOdd);

                gc.new_path();
                path.iter().for_each(|path| {
                    gc.bezier_path(path);
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
