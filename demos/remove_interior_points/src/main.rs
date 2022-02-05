use flo_curves::*;
use flo_curves::bezier::path::*;
use flo_draw::*;
use flo_draw::canvas::*;

use std::thread;
use std::time::{Duration};
use MFEKmath::{Vector, Bezier, Piecewise};

type PwBez = Piecewise<Bezier>;

fn main() {
    with_2d_graphics(|| {
        let canvas          = create_canvas_window("Remove interior points demonstration");

        let paths           = vec![
            BezierPathBuilder::<PwBez>::start(Vector { x: -14.354116, y: 66.98954 })
              .curve_to(
                (Vector { x: -8.825491, y: 59.061977 },
                 Vector { x: 2.0829022, y: 57.11726 }),
                Vector { x: 10.010462, y: 62.645885 })
              .curve_to(
                (Vector { x: 109.23956, y: 131.84758 },
                 Vector { x: 220.10713, y: 230.321 }),
                Vector { x: 346.346, y: 230.321 })
              .curve_to(
                (Vector { x: 359.739, y: 230.321 },
                 Vector { x: 372.6144, y: 229.27303 }),
                Vector { x: 385.84747, y: 227.63289 })
              .curve_to(
                (Vector { x: 395.43906, y: 226.44408 },
                 Vector { x: 404.1783, y: 233.25589 }),
                Vector { x: 405.36713, y: 242.84747 })
              .curve_to(
                (Vector { x: 406.5559, y: 252.43907 },
                 Vector { x: 399.7441, y: 261.1783 }),
                Vector { x: 390.15253, y: 262.36713 })
              .curve_to(
                (Vector { x: 375.48575, y: 264.18494 },
                 Vector { x: 361.17783, y: 265.32098 }),
                Vector { x: 346.346, y: 265.32098 })
              .curve_to(
                (Vector { x: 210.86183, y: 265.32098 },
                 Vector { x: 96.09466, y: 165.35109 }),
                Vector { x: -10.010462, y: 91.35412 })
              .curve_to(
                (Vector { x: -17.93802, y: 85.82549 },
                 Vector { x: -19.882742, y: 74.9171 }),
                Vector { x: -14.354116, y: 66.98954 })
              .build(),

            BezierPathBuilder::<PwBez>::start(Vector { x: 390.48355, y: 262.32288 })
              .curve_to(
                (Vector { x: 390.48355, y: 262.32288 },
                 Vector { x: 390.48355, y: 262.32288 }),
                Vector { x: 390.48355, y: 262.32288 })
              .curve_to(
                (Vector { x: 377.00385, y: 264.25543 },
                 Vector { x: 363.86914, y: 265.193 }),
                Vector { x: 350.21, y: 265.193 })
              .curve_to(
                (Vector { x: 236.90161, y: 265.193 },
                 Vector { x: 134.97865, y: 206.31824 }),
                Vector { x: 80.15723, y: 107.048355 })
              .curve_to(
                (Vector { x: 67.51044, y: 84.14772 },
                 Vector { x: 54.6194, y: 55.136375 }),
                Vector { x: 54.6194, y: 28.425 })
              .curve_to(
                (Vector { x: 54.6194, y: -6.1496105 },
                 Vector { x: 76.49837, y: -30.502699 }),
                Vector { x: 111.745, y: -30.5027 })
              .curve_to(
                (Vector { x: 240.37683, y: -30.502699 },
                 Vector { x: 359.86557, y: 135.03012 }),
                Vector { x: 408.73233, y: 235.33554 })
              .curve_to(
                (Vector { x: 412.9653, y: 244.02426 },
                 Vector { x: 409.35318, y: 254.49936 }),
                Vector { x: 400.66446, y: 258.73233 })
              .curve_to(
                (Vector { x: 391.97574, y: 262.9653 },
                 Vector { x: 381.50064, y: 259.35318 }),
                Vector { x: 377.26767, y: 250.66446 })
              .curve_to(
                (Vector { x: 335.21295, y: 164.34161 },
                 Vector { x: 224.40115, y: 4.4973 }),
                Vector { x: 111.745, y: 4.4973 })
              .curve_to(
                (Vector { x: 95.83539, y: 4.4973 },
                 Vector { x: 89.6194, y: 13.047722 }),
                Vector { x: 89.6194, y: 28.425 })
              .curve_to(
                (Vector { x: 89.6194, y: 48.828964 },
                 Vector { x: 101.21961, y: 72.796196 }),
                Vector { x: 110.81477, y: 90.16301 })
              .curve_to(
                (Vector { x: 159.52303, y: 178.32274 },
                 Vector { x: 249.60486, y: 230.193 }),
                Vector { x: 350.21, y: 230.193 })
              .curve_to(
                (Vector { x: 362.2078, y: 230.193 },
                 Vector { x: 373.68958, y: 229.37274 }),
                Vector { x: 385.51645, y: 227.67712 })
              .curve_to(
                (Vector { x: 395.0836, y: 226.3055 },
                 Vector { x: 403.95123, y: 232.94928 }),
                Vector { x: 405.32288, y: 242.51643 })
              .curve_to(
                (Vector { x: 406.69452, y: 252.08359 },
                 Vector { x: 400.05072, y: 260.95123 }),
                Vector { x: 390.48355, y: 262.32288 })
              .build(),

            BezierPathBuilder::<PwBez>::start(Vector { x: 400.7546, y: 258.68808 })
              .curve_to(
                (Vector { x: 392.09033, y: 262.97083 },
                 Vector { x: 381.59467, y: 259.41888 }),
                Vector { x: 377.31192, y: 250.75461 })
              .curve_to(
                (Vector { x: 326.3861, y: 147.72835 },
                 Vector { x: 275.4168, y: 44.72375 }),
                Vector { x: 224.32448, y: -58.22003 })
              .curve_to(
                (Vector { x: 224.32355, y: -58.221897 },
                 Vector { x: 224.40028, y: -58.0744 }),
                Vector { x: 224.47699, y: -57.926895 })
              .curve_to(
                (Vector { x: 224.55371, y: -57.779392 },
                 Vector { x: 224.6304, y: -57.631878 }),
                Vector { x: 224.62941, y: -57.633705 })
              .curve_to(
                (Vector { x: 196.89201, y: -108.592964 },
                 Vector { x: 125.29187, y: -279.617 }),
                Vector { x: 78.8109, y: -279.617 })
              .curve_to(
                (Vector { x: 66.41349, y: -279.617 },
                 Vector { x: 49.4785, y: -275.47705 }),
                Vector { x: 49.4785, y: -260.366 })
              .curve_to(
                (Vector { x: 49.4785, y: -167.8235 },
                 Vector { x: 303.24353, y: -62.688316 }),
                Vector { x: 377.21945, y: -24.395418 })
              .curve_to(
                (Vector { x: 428.20877, y: 1.9987056 },
                 Vector { x: 479.00412, y: 29.644754 }),
                Vector { x: 526.05536, y: 62.67728 })
              .curve_to(
                (Vector { x: 533.9656, y: 68.23069 },
                 Vector { x: 535.8761, y: 79.14512 }),
                Vector { x: 530.3227, y: 87.055336 })
              .curve_to(
                (Vector { x: 524.7693, y: 94.96555 },
                 Vector { x: 513.85486, y: 96.87613 }),
                Vector { x: 505.94467, y: 91.322716 })
              .curve_to(
                (Vector { x: 389.78024, y: 9.768924 },
                 Vector { x: 14.4785, y: -126.83389 }),
                Vector { x: 14.4785, y: -260.366 })
              .curve_to(
                (Vector { x: 14.4785, y: -296.29556 },
                 Vector { x: 46.39315, y: -314.617 }),
                Vector { x: 78.8109, y: -314.617 })
              .curve_to(
                (Vector { x: 157.64651, y: -314.617 },
                 Vector { x: 219.20493, y: -140.81 }),
                Vector { x: 255.37059, y: -74.366295 })
              .curve_to(
                (Vector { x: 255.37158, y: -74.36447 },
                 Vector { x: 255.44736, y: -74.21882 }),
                Vector { x: 255.52312, y: -74.07316 })
              .curve_to(
                (Vector { x: 255.59886, y: -73.9275 },
                 Vector { x: 255.6746, y: -73.78183 }),
                Vector { x: 255.67552, y: -73.77997 })
              .curve_to(
                (Vector { x: 306.77625, y: 29.180716 },
                 Vector { x: 357.75394, y: 132.20222 }),
                Vector { x: 408.68808, y: 235.24539 })
              .curve_to(
                (Vector { x: 412.97083, y: 243.90968 },
                 Vector { x: 409.41888, y: 254.40533 }),
                Vector { x: 400.7546, y: 258.68808 })
              .build()
        ];

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

                // let paths = path_remove_interior_points::<_, SimpleBezierPath>(&paths.iter().cloned().rev().collect(), 0.01);

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
