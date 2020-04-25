use super::fill_settings::*;
use super::super::*;
use super::super::super::*;
use super::super::super::super::geo::*;

use std::f64;
use std::ops::{Range};

///
/// Represents a collision between a ray and an object
///
#[derive(Clone)]
pub struct RayCollision<Coord, Item>
where Coord: Coordinate+Coordinate2D {
    /// Where this collision occurred
    pub position: Coord,

    /// The object that this ray colided with
    pub what: Item
}

impl<Coord, Item> RayCollision<Coord, Item>
where Coord: Coordinate+Coordinate2D {
    ///
    /// Creates a new collision at a specific point
    ///
    pub fn new(position: Coord, what: Item) -> RayCollision<Coord, Item> {
        RayCollision { position, what }
    }
}

///
/// Given a ray-casting function, traces the outline of a shape containing the specified center point 
/// 
/// `center` is a point known to be contained in the shape (it's the origin of the region to be filled) 
/// 
/// The ray-casting function has the type `Fn(Coord, Coord) -> RayList`, where the two coordinates
/// that are passed in represents the direction of the ray. It should return at least one intersection
/// along this ray. If there is an intersection, the returned list should always include the closest
/// intersection in the direction of the ray defined by the two coordinates.
///
pub fn trace_outline_convex<Coord, Item, RayList, RayFn>(center: Coord, options: &FillSettings, cast_ray: RayFn) -> Vec<RayCollision<Coord, Item>>
where   Coord:      Coordinate+Coordinate2D,
        RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
        RayFn:      Fn(Coord, Coord) -> RayList {
    trace_outline_convex_partial(center, options, (0.0)..(2.0*f64::consts::PI), cast_ray)
}

///
/// Ray traces around a specified range of angles to find the shape of the outline. Angles are in radians
///
pub (super) fn trace_outline_convex_partial<Coord, Item, RayList, RayFn>(center: Coord, options: &FillSettings, angles: Range<f64>, cast_ray: RayFn) -> Vec<RayCollision<Coord, Item>>
where   Coord:      Coordinate+Coordinate2D,
        RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
        RayFn:      Fn(Coord, Coord) -> RayList {
    // Current angle of the ray that we're casting
    let mut theta       = angles.start;

    // The number of radians moved in the last step
    let mut last_step   = 0.1;

    // The number of pixels to put between points when tracing the outline
    let step_size       = options.step;

    // Collisions we're including in the result
    let mut collisions  = vec![];

    // Cast rays until we make a complete circle
    while theta < angles.end {
        // Work out the direction of the ray
        let ray_vector      = [1.0 * theta.sin(), 1.0 * theta.cos()];
        let ray_vector      = Coord::from_components(&ray_vector);
        let ray_target      = center + ray_vector;

        // Cast this ray and get the list of collisions
        let ray_collisions  = cast_ray(center, ray_target);

        // Pick the first positive collision in the direction of the ray
        let mut nearest_collision           = None;
        let mut nearest_distance_squared    = f64::MAX;

        for ray_collision in ray_collisions {
            let collision_vector    = ray_collision.position - center;

            // Ignore collisions in the opposite direction of our ray
            let direction           = collision_vector.dot(&ray_vector);
            if direction < 0.0 { continue; }

            // If this collision is closer to the center than before, then it becomes the nearest collision
            let distance            = collision_vector.dot(&collision_vector);
            if distance < nearest_distance_squared {
                nearest_collision           = Some(ray_collision);
                nearest_distance_squared    = distance;
            }
        }

        if let Some(nearest_collision) = nearest_collision {
            // If we found a collision on this ray, add to the result
            collisions.push(nearest_collision);

            if nearest_distance_squared > 0.01 {
                // Move the ray such that we'd expect the next collision to be at approximately the distance specified by the step
                let nearest_distance    = nearest_distance_squared.sqrt();
                last_step               =  (step_size / nearest_distance).atan();
                theta                   += last_step;
            } else {
                // Collision was too close to produce a stepping angle
                theta                   += last_step;
            }
        } else {
            // Keep moving around the outline at the speed used after the last collision
            theta += last_step
        }
    }

    collisions
}


///
/// Creates a Bezier path by flood-filling a convex area whose bounds can be determined by ray-casting.
/// 
/// This won't fill areas that cannot be directly reached by a straight line from the center point. If the
/// area is not entirely closed (from the point of view of the ray-casting function), then a line will be
/// generated between the gaps.
///
pub fn flood_fill_convex<Path, Coord, Item, RayList, RayFn>(center: Coord, options: &FillSettings, cast_ray: RayFn) -> Option<Path>
where   Path:       BezierPathFactory<Point=Coord>,
        Coord:      Coordinate+Coordinate2D,
        RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
        RayFn:      Fn(Coord, Coord) -> RayList {
    // Trace where the ray casting algorithm indicates collisions with the specified center
    let collisions = trace_outline_convex(center, options, cast_ray);

    // Build a path using the LMS algorithm
    let curves = fit_curve::<Curve<Coord>>(&collisions.iter().map(|collision| collision.position.clone()).collect::<Vec<_>>(), options.fit_error);

    if let Some(curves) = curves {
        if curves.len() > 0 {
            // Convert the curves into a path
            let initial_point = curves[0].start_point();
            Some(Path::from_points(initial_point, curves.into_iter().map(|curve| {
                let (cp1, cp2)  = curve.control_points();
                let end_point   = curve.end_point();
                (cp1, cp2, end_point)
            })))
        } else {
            // No curves in the path
            None
        } 
    } else {
        // Failed to fit a curve to these points
        None
    }
}
