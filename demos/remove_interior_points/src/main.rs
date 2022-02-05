use flo_curves::*;
use flo_curves::bezier::path::*;
use flo_draw::*;
use flo_draw::canvas::*;

use std::thread;
use std::time::{Duration};
use MFEKmath::{Vector, Bezier, Piecewise};
use glifparser::{Contour};

type PwBez = Piecewise<Bezier>;

pub type Outline<PD> = Vec<Contour<PD>>;
fn main() {
    with_2d_graphics(|| {
        let canvas          = create_canvas_window("Remove interior points demonstration");
        let glif            = include_bytes!["g.glif"];
        let glif            = String::from_utf8_lossy(glif).to_string();

        // See https://github.com/Logicalshift/flo_curves/issues/14
        let point_data      = glifparser::read::<()>(&glif).unwrap();
        let bez_data        = point_data.outline.unwrap()
            .iter()
            .map(|outline| PwBez::from(outline))
            .collect::<Vec<_>>();

        let paths           = bez_data;

        loop {
            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                gc.canvas_height(1000.0);

                let bounds = paths.iter()
                    .map(|path| path.bounding_box::<Bounds<_>>())
                    .fold(Bounds::empty(), |a, b| a.union_bounds(b));
                gc.center_region(bounds.min().x as _, bounds.min().y as _, bounds.max().x as _, bounds.max().y as _);

                let paths = paths.iter()
                    .map(|path| path_remove_interior_points::<_, PwBez>(&vec![path.clone()], 0.01))
                    .fold(vec![], |a, b| path_add::<_, _, PwBez>(&a, &b, 0.01));

                // let paths = path_remove_interior_points::<_, PwBez>(&paths, 0.01);

                gc.new_path();
                paths.iter()
                    .for_each(|path| gc.bezier_path(path));

                gc.winding_rule(WindingRule::EvenOdd);

                gc.fill_color(Color::Rgba(0.0, 0.0, 0.0, 0.2));
                gc.fill();

                gc.stroke_color(Color::Rgba(0.0, 0.0, 0.6, 1.0));
                gc.stroke();

                gc.fill_color(Color::Rgba(0.0, 0.6, 0.6, 0.8));
                /*
                for (start_point, remaining_points) in paths.iter() {
                    gc.new_path();
                    gc.circle(start_point.x() as _, start_point.y() as _, 5.0);
                    gc.fill();

                    for (_, _, p) in remaining_points {
                        gc.new_path();
                        gc.circle(p.x() as _, p.y() as _, 5.0);
                        gc.fill();
                    }
                }
                */
            });

            thread::sleep(Duration::from_nanos(1_000_000_000 / 60));
        }
    });
}
