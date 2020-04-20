use flo_curves::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::path::algorithms::*;

#[test]
fn fill_convex_circle() {
    // Simple circle ray-casting algorithm
    let circle_center   = Coord2(10.0, 10.0);
    let radius          = 5.0;
    let circle_ray_cast = |from: Coord2, to: Coord2| {
        let from    = from - circle_center;
        let to      = to - circle_center;

        let x1      = from.x();
        let y1      = from.y();
        let x2      = to.x();
        let y2      = to.y();

        let dx      = x2-x1;
        let dy      = y2-y1;
        let dr      = (dx*dx + dy*dy).sqrt();

        let d       = x1*y2 - x2*y1;

        let xc1     = (d*dy + dy.signum()*dx)*((radius*radius*dr*dr - d*d).sqrt())/(dr*dr);
        let xc2     = (d*dy - dy.signum()*dx)*((radius*radius*dr*dr - d*d).sqrt())/(dr*dr);
        let yc1     = (-d*dx + dy.abs())*((radius*radius*dr*dr - d*d).sqrt())/(dr*dr);
        let yc2     = (-d*dx - dy.abs())*((radius*radius*dr*dr - d*d).sqrt())/(dr*dr);

        vec![
            RayCollision::new(Coord2(xc1, yc1)+circle_center, ()), RayCollision::new(Coord2(xc2, yc2)+circle_center, ())
        ]
    };

    // Flood-fill this curve
    let path: Option<SimpleBezierPath> = flood_fill_convex(circle_center, &FillOptions::default(), circle_ray_cast);

    assert!(path.is_some());

    for curve in path.unwrap().to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t           = (t as f64)/100.0;
            let distance    = circle_center.distance_to(&curve.point_at_pos(t));

            assert!((distance-radius).abs() < 1.0);
        }
    }
}
