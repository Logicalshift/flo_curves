use flo_curves::*;
use flo_curves::bezier::path::*;
use flo_draw::*;
use flo_draw::canvas::*;

use std::thread;
use std::time::{Duration};

fn main() {
    with_2d_graphics(|| {
        let canvas          = create_canvas_window("Remove interior points demonstration");

        let paths           = vec![
            BezierPathBuilder::<SimpleBezierPath>::start(Coord2(-14.354116, 66.98954))
              .curve_to(
                (Coord2(-8.825491, 59.061977),
                 Coord2(2.0829022, 57.11726)),
                Coord2(10.010462, 62.645885))
              .curve_to(
                (Coord2(109.23956, 131.84758),
                 Coord2(220.10713, 230.321)),
                Coord2(346.346, 230.321))
              .curve_to(
                (Coord2(359.739, 230.321),
                 Coord2(372.6144, 229.27303)),
                Coord2(385.84747, 227.63289))
              .curve_to(
                (Coord2(395.43906, 226.44408),
                 Coord2(404.1783, 233.25589)),
                Coord2(405.36713, 242.84747))
              .curve_to(
                (Coord2(406.5559, 252.43907),
                 Coord2(399.7441, 261.1783)),
                Coord2(390.15253, 262.36713))
              .curve_to(
                (Coord2(375.48575, 264.18494),
                 Coord2(361.17783, 265.32098)),
                Coord2(346.346, 265.32098))
              .curve_to(
                (Coord2(210.86183, 265.32098),
                 Coord2(96.09466, 165.35109)),
                Coord2(-10.010462, 91.35412))
              .curve_to(
                (Coord2(-17.93802, 85.82549),
                 Coord2(-19.882742, 74.9171)),
                Coord2(-14.354116, 66.98954))
              .build(),

            BezierPathBuilder::<SimpleBezierPath>::start(Coord2(390.48355, 262.32288))
              .curve_to(
                (Coord2(390.48355, 262.32288),
                 Coord2(390.48355, 262.32288)),
                Coord2(390.48355, 262.32288))
              .curve_to(
                (Coord2(377.00385, 264.25543),
                 Coord2(363.86914, 265.193)),
                Coord2(350.21, 265.193))
              .curve_to(
                (Coord2(236.90161, 265.193),
                 Coord2(134.97865, 206.31824)),
                Coord2(80.15723, 107.048355))
              .curve_to(
                (Coord2(67.51044, 84.14772),
                 Coord2(54.6194, 55.136375)),
                Coord2(54.6194, 28.425))
              .curve_to(
                (Coord2(54.6194, -6.1496105),
                 Coord2(76.49837, -30.502699)),
                Coord2(111.745, -30.5027))
              .curve_to(
                (Coord2(240.37683, -30.502699),
                 Coord2(359.86557, 135.03012)),
                Coord2(408.73233, 235.33554))
              .curve_to(
                (Coord2(412.9653, 244.02426),
                 Coord2(409.35318, 254.49936)),
                Coord2(400.66446, 258.73233))
              .curve_to(
                (Coord2(391.97574, 262.9653),
                 Coord2(381.50064, 259.35318)),
                Coord2(377.26767, 250.66446))
              .curve_to(
                (Coord2(335.21295, 164.34161),
                 Coord2(224.40115, 4.4973)),
                Coord2(111.745, 4.4973))
              .curve_to(
                (Coord2(95.83539, 4.4973),
                 Coord2(89.6194, 13.047722)),
                Coord2(89.6194, 28.425))
              .curve_to(
                (Coord2(89.6194, 48.828964),
                 Coord2(101.21961, 72.796196)),
                Coord2(110.81477, 90.16301))
              .curve_to(
                (Coord2(159.52303, 178.32274),
                 Coord2(249.60486, 230.193)),
                Coord2(350.21, 230.193))
              .curve_to(
                (Coord2(362.2078, 230.193),
                 Coord2(373.68958, 229.37274)),
                Coord2(385.51645, 227.67712))
              .curve_to(
                (Coord2(395.0836, 226.3055),
                 Coord2(403.95123, 232.94928)),
                Coord2(405.32288, 242.51643))
              .curve_to(
                (Coord2(406.69452, 252.08359),
                 Coord2(400.05072, 260.95123)),
                Coord2(390.48355, 262.32288))
              .build(),

            BezierPathBuilder::<SimpleBezierPath>::start(Coord2(400.7546, 258.68808))
              .curve_to(
                (Coord2(392.09033, 262.97083),
                 Coord2(381.59467, 259.41888)),
                Coord2(377.31192, 250.75461))
              .curve_to(
                (Coord2(326.3861, 147.72835),
                 Coord2(275.4168, 44.72375)),
                Coord2(224.32448, -58.22003))
              .curve_to(
                (Coord2(224.32355, -58.221897),
                 Coord2(224.40028, -58.0744)),
                Coord2(224.47699, -57.926895))
              .curve_to(
                (Coord2(224.55371, -57.779392),
                 Coord2(224.6304, -57.631878)),
                Coord2(224.62941, -57.633705))
              .curve_to(
                (Coord2(196.89201, -108.592964),
                 Coord2(125.29187, -279.617)),
                Coord2(78.8109, -279.617))
              .curve_to(
                (Coord2(66.41349, -279.617),
                 Coord2(49.4785, -275.47705)),
                Coord2(49.4785, -260.366))
              .curve_to(
                (Coord2(49.4785, -167.8235),
                 Coord2(303.24353, -62.688316)),
                Coord2(377.21945, -24.395418))
              .curve_to(
                (Coord2(428.20877, 1.9987056),
                 Coord2(479.00412, 29.644754)),
                Coord2(526.05536, 62.67728))
              .curve_to(
                (Coord2(533.9656, 68.23069),
                 Coord2(535.8761, 79.14512)),
                Coord2(530.3227, 87.055336))
              .curve_to(
                (Coord2(524.7693, 94.96555),
                 Coord2(513.85486, 96.87613)),
                Coord2(505.94467, 91.322716))
              .curve_to(
                (Coord2(389.78024, 9.768924),
                 Coord2(14.4785, -126.83389)),
                Coord2(14.4785, -260.366))
              .curve_to(
                (Coord2(14.4785, -296.29556),
                 Coord2(46.39315, -314.617)),
                Coord2(78.8109, -314.617))
              .curve_to(
                (Coord2(157.64651, -314.617),
                 Coord2(219.20493, -140.81)),
                Coord2(255.37059, -74.366295))
              .curve_to(
                (Coord2(255.37158, -74.36447),
                 Coord2(255.44736, -74.21882)),
                Coord2(255.52312, -74.07316))
              .curve_to(
                (Coord2(255.59886, -73.9275),
                 Coord2(255.6746, -73.78183)),
                Coord2(255.67552, -73.77997))
              .curve_to(
                (Coord2(306.77625, 29.180716),
                 Coord2(357.75394, 132.20222)),
                Coord2(408.68808, 235.24539))
              .curve_to(
                (Coord2(412.97083, 243.90968),
                 Coord2(409.41888, 254.40533)),
                Coord2(400.7546, 258.68808))
              .build()
        ];

        loop {
            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                gc.canvas_height(1000.0);

                let bounds = paths.iter()
                    .map(|path| path.bounding_box::<Bounds<_>>())
                    .fold(Bounds::empty(), |a, b| a.union_bounds(b));
                gc.center_region(bounds.min().0 as _, bounds.min().1 as _, bounds.max().0 as _, bounds.max().1 as _);

                let paths = paths.iter()
                    .map(|path| path_remove_interior_points::<_, SimpleBezierPath>(&vec![path.clone()], 0.01))
                    .fold(vec![], |a, b| path_add::<_, _, SimpleBezierPath>(&a, &b, 0.01));

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
            });

            thread::sleep(Duration::from_nanos(1_000_000_000 / 60));
        }
    });
}
