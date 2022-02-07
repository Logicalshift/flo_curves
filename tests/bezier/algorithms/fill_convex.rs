use flo_curves::bezier::path::algorithms::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::*;

fn circle_ray_cast(
    circle_center: Coord2,
    radius: f64,
) -> impl Fn(Coord2, Coord2) -> Vec<RayCollision<Coord2, ()>> {
    move |from: Coord2, to: Coord2| {
        let from = from - circle_center;
        let to = to - circle_center;

        let x1 = from.x();
        let y1 = from.y();
        let x2 = to.x();
        let y2 = to.y();

        let dx = x2 - x1;
        let dy = y2 - y1;
        let dr = (dx * dx + dy * dy).sqrt();

        let d = x1 * y2 - x2 * y1;

        let xc1 = (d * dy + (dy.signum() * dx * ((radius * radius * dr * dr - d * d).sqrt())))
            / (dr * dr);
        let xc2 = (d * dy - (dy.signum() * dx * ((radius * radius * dr * dr - d * d).sqrt())))
            / (dr * dr);
        let yc1 = (-d * dx + (dy.abs() * ((radius * radius * dr * dr - d * d).sqrt()))) / (dr * dr);
        let yc2 = (-d * dx - (dy.abs() * ((radius * radius * dr * dr - d * d).sqrt()))) / (dr * dr);

        vec![
            RayCollision::new(Coord2(xc1, yc1) + circle_center, ()),
            RayCollision::new(Coord2(xc2, yc2) + circle_center, ()),
        ]
    }
}

#[test]
fn trace_convex_circle() {
    // Simple circle ray-casting algorithm
    let circle_center = Coord2(10.0, 10.0);
    let radius = 100.0;
    let circle_ray_cast = circle_ray_cast(circle_center, radius);

    // Trace the outline
    let outline = trace_outline_convex(circle_center, &FillSettings::default(), circle_ray_cast);

    // Should be at least one point
    assert!(outline.len() > 0);

    // Points should be no more that 4.0 pixels apart and should be the correct distance from the circle
    for point_idx in 0..outline.len() {
        let next_point_idx = if point_idx + 1 >= outline.len() {
            0
        } else {
            point_idx + 1
        };
        let point = &outline[point_idx];
        let next_point = &outline[next_point_idx];

        assert!((point.position.distance_to(&circle_center) - radius).abs() < 1.0);
        assert!(point.position.distance_to(&next_point.position) <= 4.0);
    }

    assert!(outline.len() > 8);
}

#[test]
fn fill_convex_circle() {
    // Simple circle ray-casting algorithm
    let circle_center = Coord2(10.0, 10.0);
    let radius = 100.0;
    let circle_ray_cast = circle_ray_cast(circle_center, radius);

    // Flood-fill this curve
    let path: Option<SimpleBezierPath> =
        flood_fill_convex(circle_center, &FillSettings::default(), circle_ray_cast);

    assert!(path.is_some());

    for curve in path.unwrap().to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t = (t as f64) / 100.0;
            let distance = circle_center.distance_to(&curve.point_at_pos(t));

            assert!((distance - radius).abs() < 1.0);
        }
    }
}

#[test]
fn trace_convex_doughnut() {
    // With a convex fill, a 'doughnut' shape will only fill those points that are immediately reachable from the origin point
    let circle_center = Coord2(10.0, 10.0);
    let outer_radius = 100.0;
    let inner_radius = 50.0;
    let outer_circle = circle_ray_cast(circle_center, outer_radius);
    let inner_circle = circle_ray_cast(circle_center, inner_radius);
    let doughnut = |from: Coord2, to: Coord2| {
        outer_circle(from, to)
            .into_iter()
            .chain(inner_circle(from, to))
    };

    // Trace the outline
    let start_point = circle_center + Coord2(inner_radius + 10.0, 0.0);
    let outline = trace_outline_convex(start_point, &FillSettings::default(), doughnut);

    // Should be at least one point
    assert!(outline.len() > 0);

    for point_idx in 0..outline.len() {
        let point = &outline[point_idx];

        assert!(
            (point.position.distance_to(&circle_center) - outer_radius).abs() < 1.0
                || (point.position.distance_to(&circle_center) - inner_radius).abs() < 1.0
        );
    }

    assert!(outline.len() > 8);
}

#[test]
fn fill_convex_doughnut() {
    // With a convex fill, a 'doughnut' shape will only fill those points that are immediately reachable from the origin point
    let circle_center = Coord2(10.0, 10.0);
    let outer_radius = 100.0;
    let inner_radius = 50.0;
    let outer_circle = circle_ray_cast(circle_center, outer_radius);
    let inner_circle = circle_ray_cast(circle_center, inner_radius);
    let doughnut = |from: Coord2, to: Coord2| {
        outer_circle(from, to)
            .into_iter()
            .chain(inner_circle(from, to))
    };

    // Flood-fill this curve
    let start_point = circle_center + Coord2(inner_radius + 10.0, 0.0);
    let path = flood_fill_convex::<SimpleBezierPath, _, _, _, _>(
        start_point,
        &FillSettings::default(),
        doughnut,
    );

    assert!(path.is_some());

    for curve in path.as_ref().unwrap().to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t = (t as f64) / 100.0;
            let distance = circle_center.distance_to(&curve.point_at_pos(t));

            assert!(distance >= inner_radius - 2.0 && distance <= outer_radius + 2.0)
        }
    }
}
