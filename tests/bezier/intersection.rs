use flo_curves::*;
use flo_curves::bezier;
use flo_curves::line;

#[test]
fn find_intersection_on_straight_line() {
    // Cross that intersects at (5.0, 5.0)
    let line    = (Coord2(0.0, 0.0), Coord2(10.0, 10.0));
    let curve   = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 0.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_line(&curve, &line);
    assert!(intersections.len() == 1);

    let intersect_point = curve.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.01);
}

#[test]
fn find_intersection_with_vertical_ray() {
    // Cross that intersects at (5.0, 5.0)
    let line    = (Coord2(5.0, 0.0), Coord2(5.0, 10.0));
    let curve   = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 0.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_line(&curve, &line);
    assert!(intersections.len() == 1);

    let intersect_point = curve.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.01);
}

#[test]
fn find_intersection_with_horizontal_ray() {
    // Cross that intersects at (5.0, 5.0)
    let line    = (Coord2(0.0, 5.0), Coord2(10.0, 5.0));
    let curve   = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 0.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_line(&curve, &line);
    assert!(intersections.len() == 1);

    let intersect_point = curve.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.01);
}

#[test]
fn no_intersection_if_line_does_not_cross_curve() {
    // Line moves away from the curve
    let line    = (Coord2(0.0, 0.0), Coord2(-10.0, -10.0));
    let curve   = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 0.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_line(&curve, &line);
    assert!(intersections.len() == 0);
}

#[test]
fn find_intersection_on_straight_line_against_ray() {
    // Line moves away from the curve so it doesn't intersect. When we use intersects_ray(), however, we find intersections anywhere along the line
    let line    = (Coord2(0.0, 0.0), Coord2(-10.0, -10.0));
    let curve   = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 0.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_ray(&curve, &line);
    assert!(intersections.len() == 1);

    let intersect_point = curve.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.01);
}

#[test]
fn find_intersection_on_curve() {
    let line    = (Coord2(0.0, 6.0), Coord2(10.0, 4.0));
    let curve   = bezier::Curve {
        start_point:    Coord2(0.0, 2.0),
        end_point:      Coord2(10.0, 8.0),
        control_points: (Coord2(0.0, 20.0), Coord2(10.0, -10.0))
    };

    // Find the intersections
    let intersections   = bezier::curve_intersects_line(&curve, &line);

    // Should be 3 intersections
    assert!(intersections.len() == 3);

    // Curve is symmetrical so the mid-point should be at 5,5
    assert!(curve.point_at_pos(intersections[1].0).distance_to(&Coord2(5.0, 5.0)) < 0.01);

    // Other points are a bit less precise
    assert!(curve.point_at_pos(intersections[0].0).distance_to(&Coord2(0.260, 5.948)) < 0.01);
    assert!(curve.point_at_pos(intersections[2].0).distance_to(&Coord2(9.740, 4.052)) < 0.01);
}

#[test]
fn find_intersection_on_curve_short_line() {
    let line    = (Coord2(0.0, 6.0), Coord2(8.0, 4.4));
    let curve   = bezier::Curve {
        start_point:    Coord2(0.0, 2.0),
        end_point:      Coord2(10.0, 8.0),
        control_points: (Coord2(0.0, 20.0), Coord2(10.0, -10.0))
    };

    // Find the intersections
    let intersections   = bezier::curve_intersects_line(&curve, &line);

    // Should be 2 intersections
    assert!(intersections.len() == 2);

    assert!(curve.point_at_pos(intersections[1].0).distance_to(&Coord2(5.0, 5.0)) < 0.01);
    assert!(curve.point_at_pos(intersections[0].0).distance_to(&Coord2(0.260, 5.948)) < 0.01);
}

#[test]
fn dot_intersects_nothing() {
    // Line with 0 length
    let line    = (Coord2(4.0, 4.0), Coord2(4.0, 4.0));
    let curve   = bezier::Curve {
        start_point:    Coord2(0.0, 2.0),
        end_point:      Coord2(10.0, 8.0),
        control_points: (Coord2(0.0, 20.0), Coord2(10.0, -10.0))
    };

    // Find the intersections
    let intersections   = bezier::curve_intersects_line(&curve, &line);

    // Should be no intersections
    assert!(intersections.len() == 0);
}

#[test]
fn lines_intersect_at_start() {
    let line1   = (Coord2(4.0, 4.0), Coord2(5.0, 8.0));
    let line2   = (Coord2(4.0, 4.0), Coord2(8.0, 5.0));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&line2);

    let intersections = bezier::curve_intersects_line(&curve2, &line1);

    assert!(intersections.len() == 1);
    assert!(intersections[0].0 < 0.01);
    assert!(curve2.point_at_pos(intersections[0].0).distance_to(&Coord2(4.0, 4.0)) < 0.01);
}

#[test]
fn lines_intersect_at_end() {
    let line1   = (Coord2(5.0, 8.0), Coord2(4.0, 4.0));
    let line2   = (Coord2(8.0, 5.0), Coord2(4.0, 4.0));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&line2);

    let intersections = bezier::curve_intersects_line(&curve2, &line1);

    assert!(intersections.len() == 1);
    assert!(intersections[0].0 > 0.99);
    assert!(curve2.point_at_pos(intersections[0].0).distance_to(&Coord2(4.0, 4.0)) < 0.01);
}

#[test]
fn lines_intersect_start_to_end() {
    let line1   = (Coord2(4.0, 4.0), Coord2(5.0, 8.0));
    let line2   = (Coord2(8.0, 5.0), Coord2(4.0, 4.0));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&line2);

    let intersections = bezier::curve_intersects_line(&curve2, &line1);

    assert!(intersections.len() == 1);
    assert!(intersections[0].0 > 0.99);
    assert!(curve2.point_at_pos(intersections[0].0).distance_to(&Coord2(4.0, 4.0)) < 0.01);
}

#[test]
fn ray_intersects_collinear_line_1() {
    // Ray intersecting a collinear line edge-on
    let ray     = (Coord2(0.0, 0.0), Coord2(2.0, 1.0));
    let line    = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(4.0, 2.0), Coord2(8.0, 4.0)));

    let intersections = bezier::curve_intersects_ray(&line, &ray);

    assert!(intersections.len() == 2);
    assert!(intersections[0].0 < 0.001);
    assert!(intersections[0].2.distance_to(&Coord2(4.0, 2.0)) < 0.01);
    assert!((intersections[1].0-1.0).abs() < 0.001);
    assert!(intersections[1].2.distance_to(&Coord2(8.0, 4.0)) < 0.01);
}

#[test]
fn ray_intersects_collinear_line_2() {
    // Intersecting a collinear line which has a point closer to the start of the ray than the start of the line
    let ray     = (Coord2(0.0, 0.0), Coord2(2.0, 1.0));
    let line    = bezier::Curve::from_points(Coord2(4.0, 2.0), (Coord2(2.0, 1.0), Coord2(10.0, 5.0)), Coord2(8.0, 4.0)); // line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(4.0, 2.0), Coord2(8.0, 4.0)));

    let intersections = bezier::curve_intersects_ray(&line, &ray);

    assert!(intersections.len() == 2);
    assert!(intersections[0].0 < 0.001);
    assert!(intersections[0].2.distance_to(&Coord2(4.0, 2.0)) < 0.01);
    assert!((intersections[1].0-1.0).abs() < 0.001);
    assert!(intersections[1].2.distance_to(&Coord2(8.0, 4.0)) < 0.01);
}

#[test]
fn ray_intersects_collinear_line_3() {
    // Line moving towards the start of the ray instead of away from it
    let ray     = (Coord2(0.0, 0.0), Coord2(2.0, 1.0));
    let line    = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(8.0, 4.0), Coord2(4.0, 2.0)));

    let intersections = bezier::curve_intersects_ray(&line, &ray);

    assert!(intersections.len() == 2);
    assert!(intersections[0].0.abs() < 0.001);
    assert!(intersections[0].2.distance_to(&Coord2(8.0, 4.0)) < 0.01);
    assert!((intersections[1].0-1.0).abs() < 0.001);
    assert!(intersections[1].2.distance_to(&Coord2(4.0, 2.0)) < 0.01);
}

#[test]
fn ray_intersects_curve_1() {
    // Failed intersection in ring_with_offset_crossbar_ray_casting_issue
    let curve   = bezier::Curve::from_points(Coord2(0.5857864376269051, 0.5857864376269049), (Coord2(0.488017920077567, 0.683554955176243), Coord2(0.40248767198507907, 0.7889273585090868)), Coord2(0.3291956933494412, 0.899999999999999));
    let ray     = (Coord2(0.3853378796624052, 0.7560017173290998), Coord2(0.385337879662404, 1.0999999999999999));

    let intersections = bezier::curve_intersects_ray(&curve, &ray);

    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);
}

#[test]
fn ray_intersects_curve_1a() {
    // As above but the ray is less vertical
    let curve   = bezier::Curve::from_points(Coord2(0.5857864376269051, 0.5857864376269049), (Coord2(0.488017920077567, 0.683554955176243), Coord2(0.40248767198507907, 0.7889273585090868)), Coord2(0.3291956933494412, 0.899999999999999));
    let ray     = (Coord2(0.3854378796624052, 0.7560017173290998), Coord2(0.385337879662404, 1.0999999999999999));

    let intersections = bezier::curve_intersects_ray(&curve, &ray);

    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);
}

#[test]
fn ray_intersects_curve_1b() {
    // Failed intersection in ring_with_offset_crossbar_ray_casting_issue (different ray: all vertical rays seem to be an issue)
    let curve   = bezier::Curve::from_points(Coord2(0.5857864376269051, 0.5857864376269049), (Coord2(0.488017920077567, 0.683554955176243), Coord2(0.40248767198507907, 0.7889273585090868)), Coord2(0.3291956933494412, 0.899999999999999));
    let ray     = (Coord2(0.395337879662404, 0.7560017173290998), Coord2(0.395337879662404, 1.0999999999999999));

    let intersections = bezier::curve_intersects_ray(&curve, &ray);

    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);
}

#[test]
fn ray_intersects_curve_1c() {
    // Horizontal ray to the collision point in 1a
    let curve   = bezier::Curve::from_points(Coord2(0.5857864376269051, 0.5857864376269049), (Coord2(0.488017920077567, 0.683554955176243), Coord2(0.40248767198507907, 0.7889273585090868)), Coord2(0.3291956933494412, 0.899999999999999));
    let ray     = (Coord2(0.7560017173290998, 0.8192109187049827), Coord2(1.0999999999999999, 0.8192109187049827));

    let intersections = bezier::curve_intersects_ray(&curve, &ray);

    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);
}

#[test]
fn ray_intersects_curve_1d() {
    // Failed intersection in ring_with_offset_crossbar_ray_casting_issue (vertical ray to the collision point from 1a)
    let curve   = bezier::Curve::from_points(Coord2(0.5857864376269051, 0.5857864376269049), (Coord2(0.488017920077567, 0.683554955176243), Coord2(0.40248767198507907, 0.7889273585090868)), Coord2(0.3291956933494412, 0.899999999999999));
    let ray     = (Coord2(0.38541950989400653, 0.7560017173290998), Coord2(0.38541950989400653, 1.0999999999999999));

    let intersections = bezier::curve_intersects_ray(&curve, &ray);

    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);
}

#[test]
fn ray_intersects_curve_1e() {
    // Same intersection but using the clipping algorithm (fails the same way as we eventually try to find via the root finder)
    let curve   = bezier::Curve::from_points(Coord2(0.5857864376269051, 0.5857864376269049), (Coord2(0.488017920077567, 0.683554955176243), Coord2(0.40248767198507907, 0.7889273585090868)), Coord2(0.3291956933494412, 0.899999999999999));
    let ray     = (Coord2(0.3853378796624052, 0.0), Coord2(0.385337879662404, 10.0));
    let ray     = line::line_to_bezier(&ray);

    let intersections = bezier::curve_intersects_curve_clip(&curve, &ray, 0.01);

    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);
}

#[test]
fn roots_library_does_not_have_missing_root_bug() {
    use roots::*;

    // Known root of a set of coefficients (which happen to be the coefficients from the failing tests above)
    let a = -0.000000000000000040410628481035;
    let b = 0.0126298310280606;
    let c = -0.100896606408756;
    let d = 0.0689539597036461;

    let x = 0.754710877053;

    // Demonstrate that this is a root
    assert!((a*x*x*x + b*x*x + c*x + d).abs() < 0.001);

    // Try to find this root
    let roots = find_roots_cubic(a, b, c, d);
    let roots = match roots {
        Roots::No(_)    => vec![],
        Roots::One(r)   => r.to_vec(),
        Roots::Two(r)   => r.to_vec(),
        Roots::Three(r) => r.to_vec(),
        Roots::Four(r)  => r.to_vec()
    };

    // Should exist a root that's close to the value above
    println!("{:?}", roots);
    assert!(roots.into_iter().any(|r| (r-x).abs() < 0.01));
}

#[test]
fn ray_missing_root_2() {
    use roots::*;

    // As above but with the slightly weird coefficent a set to 0.0
    let a = -0.0;
    let b = 0.0126298310280606;
    let c = -0.100896606408756;
    let d = 0.0689539597036461;

    let x = 0.754710877053;

    // Demonstrate that this is a root
    assert!((a*x*x*x + b*x*x + c*x + d).abs() < 0.001);

    // Try to find this root
    let roots = find_roots_cubic(a, b, c, d);
    let roots = match roots {
        Roots::No(_)    => vec![],
        Roots::One(r)   => r.to_vec(),
        Roots::Two(r)   => r.to_vec(),
        Roots::Three(r) => r.to_vec(),
        Roots::Four(r)  => r.to_vec()
    };

    // Should exist a root that's close to the value above
    println!("{:?}", roots);
    assert!(roots.into_iter().any(|r| (r-x).abs() < 0.01));
}

#[test]
fn ray_missing_root_3() {
    use roots::*;

    // Again, but with the smallest value of a that we get a sensible answer for
    let a = -0.0000000002;
    let b = 0.0126298310280606;
    let c = -0.100896606408756;
    let d = 0.0689539597036461;

    let x = 0.754710877053;

    // Demonstrate that this is a root
    assert!((a*x*x*x + b*x*x + c*x + d).abs() < 0.001);

    // Try to find this root
    let roots = find_roots_cubic(a, b, c, d);
    let roots = match roots {
        Roots::No(_)    => vec![],
        Roots::One(r)   => r.to_vec(),
        Roots::Two(r)   => r.to_vec(),
        Roots::Three(r) => r.to_vec(),
        Roots::Four(r)  => r.to_vec()
    };

    // Should exist a root that's close to the value above
    println!("{:?}", roots);
    assert!(roots.into_iter().any(|r| (r-x).abs() < 0.01));
}

#[test]
fn collide_close_to_circle_1() {
    use flo_curves::line::{line_to_bezier};
    use flo_curves::bezier::{Curve, curve_intersects_ray, curve_intersects_curve_clip};
    use flo_curves::arc::*;
    use flo_curves::bezier::path::*;

    // This was found to produce a bad set of collisions in `remove_interior_for_ring_with_offset_crossbar_removes_center_1`
    // `close_line` here passes very close to the circle, and produced a collision both where it intersected the line and at the end, creating an odd subpath
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let close_line  = (Coord2(0.2, 1.1), Coord2(3.8, 1.1));

    // Try using curve_intersects_ray first: should produce at most one intersection per line
    for curve in ring1.to_curves::<Curve<_>>() {
        let collisions = curve_intersects_ray(&curve, &close_line);
        println!("{:?}", collisions);
        assert!(collisions.len() <= 1);
    }

    // Try using curve_intersects_line next: should produce at most one intersection per line
    let close_curve = line_to_bezier::<_, Curve<_>>(&close_line);

    for curve in ring1.to_curves::<Curve<_>>() {
        let collisions = curve_intersects_curve_clip(&curve, &close_curve, 0.01);
        println!("{:?}", collisions);
        assert!(collisions.len() <= 1);

        let collisions = curve_intersects_curve_clip(&close_curve, &curve, 0.01);
        println!("{:?}", collisions);
        assert!(collisions.len() <= 1);
    }
}

#[test]
fn collide_close_to_circle_2() {
    use flo_curves::line::{line_to_bezier};
    use flo_curves::bezier::{Curve, curve_intersects_ray, curve_intersects_curve_clip};
    use flo_curves::arc::*;
    use flo_curves::bezier::path::*;

    // This was found to produce a bad set of collisions in `remove_interior_for_ring_with_offset_crossbar_removes_center_1`
    // `close_line` here passes very close to the circle, and produced a collision both where it intersected the line and at the end, creating an odd subpath
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let close_line  = (Coord2(3.8, 1.1), Coord2(3.8, 0.9));

    // Try using curve_intersects_ray first: should produce at most one intersection per line
    for curve in ring1.to_curves::<Curve<_>>() {
        let collisions = curve_intersects_ray(&curve, &close_line);
        println!("{:?}", collisions);
        //assert!(collisions.len() <= 1);
    }

    // Try using curve_intersects_line next: this line does not intersect (it's quite close to the curve but does not actually intersect it)
    let close_curve = line_to_bezier::<_, Curve<_>>(&close_line);

    for curve in ring1.to_curves::<Curve<_>>() {
        let collisions = curve_intersects_curve_clip(&curve, &close_curve, 0.01);
        println!("{:?}", collisions);
        assert!(collisions.len() == 0);

        let collisions = curve_intersects_curve_clip(&close_curve, &curve, 0.01);
        println!("{:?}", collisions);
        assert!(collisions.len() == 0);
    }
}
