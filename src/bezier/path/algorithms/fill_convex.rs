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
where
    Coord: Coordinate+Coordinate2D,
{
    /// Where this collision occurred
    pub position: Coord,

    /// The object that this ray colided with
    pub what: Item
}

impl<Coord, Item> RayCollision<Coord, Item>
where
    Coord: Coordinate+Coordinate2D,
{
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
where
    Coord:      Coordinate+Coordinate2D,
    RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
    RayFn:      Fn(Coord, Coord) -> RayList,
{
    trace_outline_convex_partial(center, options, (0.0)..(2.0*f64::consts::PI), cast_ray)
}

///
/// Finds the nearest collision and the square of its distance from the center from the results of a ray-casting operation
///
fn find_nearest_collision<Coord, Item, RayList>(candidates: RayList, center: Coord, ray_vector: Coord) -> Option<(RayCollision<Coord, Item>, f64)>
where
    Coord:      Coordinate+Coordinate2D,
    RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
{
    // Pick the first positive collision in the direction of the ray
    let mut nearest_collision           = None;
    let mut nearest_distance_squared    = f64::MAX;

    for ray_collision in candidates {
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

    nearest_collision.map(|nearest_collision| (nearest_collision, nearest_distance_squared))
}

///
/// Performs a raycast from a center point at a particular angle
///
fn perform_ray_cast<Coord, Item, RayList, RayFn>(center: Coord, theta: f64, cast_ray: RayFn) -> Option<(RayCollision<Coord, Item>, f64)>
where
    Coord:      Coordinate+Coordinate2D,
    RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
    RayFn:      Fn(Coord, Coord) -> RayList,
{
    // Work out the direction of the ray
    let ray_vector          = [1.0 * theta.sin(), 1.0 * theta.cos()];
    let ray_vector          = Coord::from_components(&ray_vector);
    let ray_target          = center + ray_vector;

    // Cast this ray and get the list of collisions
    let ray_collisions      = cast_ray(center, ray_target);

    // Pick the first positive collision in the direction of the ray
    find_nearest_collision(ray_collisions, center, ray_vector)
}

///
/// Ray traces around a specified range of angles to find the shape of the outline. Angles are in radians
///
pub (super) fn trace_outline_convex_partial<Coord, Item, RayList, RayFn>(center: Coord, options: &FillSettings, angles: Range<f64>, cast_ray: RayFn) -> Vec<RayCollision<Coord, Item>>
where
    Coord:      Coordinate+Coordinate2D,
    RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
    RayFn:      Fn(Coord, Coord) -> RayList,
{
    // The minimum number of radians to move forward when a ray does not find a collision
    let min_step            = 0.02;

    // The number of pixels to put between points when tracing the outline
    let step_size           = options.step;
    let max_step            = step_size * 2.0;
    let max_step_squared    = max_step * max_step;

    // Collisions we're including in the result
    let mut collisions      = vec![];

    // Create a stack to track the state
    let mut stack           = vec![];
    struct StackEntry<Coord: Coordinate+Coordinate2D, Item> {
        angle:      Range<f64>,
        start_pos:  Option<(RayCollision<Coord, Item>, f64)>,
        end_pos:    Option<Coord>
    }

    // Ray cast a few points to get the initial stack of points to check
    for check_point in 0..4 {
        let check_point = (3-check_point) as f64;
        let theta       = angles.start + (angles.end-angles.start)/4.0 * check_point;
        let end_theta   = theta + (angles.end-angles.start)/4.0;

        let start_pos   = perform_ray_cast(center, theta, &cast_ray);
        let end_pos     = perform_ray_cast(center, end_theta, &cast_ray);

        stack.push(StackEntry {
            angle:      theta..end_theta,
            start_pos:  start_pos,
            end_pos:    end_pos.map(|(end_pos, _distance)| end_pos.position)
        });
    }

    // Divide up the check points until the gap between them becomes small enough that it's less than the maximum gap size
    while let Some(entry) = stack.pop() {
        if let (Some((start_pos, _start_distance_squared)), Some(end_pos)) = (entry.start_pos.as_ref(), entry.end_pos) {
            // Check the distance between the start and the end
            let offset              = end_pos - start_pos.position;
            let distance_squared    = offset.dot(&offset);

            if distance_squared < max_step_squared {
                // This point is close enough to its following point to be added to the result
                collisions.push(entry.start_pos.unwrap().0);
            } else {
                // Divide the entry into two by casting a ray between the two points
                let mid_point   = (entry.angle.start + entry.angle.end) / 2.0;
                let mid_ray     = perform_ray_cast(center, mid_point, &cast_ray);
                let mid_ray_pos = mid_ray.as_ref().map(|(collision, _)| collision.position);

                // If there's a discontinuity (eg, a corner we can't see around), we'll see that the mid point is very close to the end point and far from the start point
                if let Some(mid_ray_pos) = mid_ray_pos {
                    // Compute the distance from the start to the mid-point and the mid-point to the end
                    let mid_to_end          = end_pos - mid_ray_pos;
                    let mid_to_end_sq       = mid_to_end.dot(&mid_to_end);

                    // If the end is very close to the mid-point ...
                    if mid_to_end_sq < (step_size * step_size) {
                        // ... and is over 3/4 from the start point ...
                        let three_quarters_sq   = (9.0*distance_squared)/16.0;
                        let start_to_mid        = mid_ray_pos - start_pos.position;
                        let start_to_mid_sq     = start_to_mid.dot(&start_to_mid);

                        if start_to_mid_sq >= three_quarters_sq {
                            // ... we've hit an edge and won't be able to find a point closer to the start position
                            collisions.push(entry.start_pos.unwrap().0);
                            continue;
                        }
                    }
                }

                // Divide into two pairs of ranges (process the earlier one first)
                stack.push(StackEntry {
                    angle:      mid_point..entry.angle.end,
                    start_pos:  mid_ray,
                    end_pos:    entry.end_pos
                });
                stack.push(StackEntry {
                    angle:      entry.angle.start..mid_point,
                    start_pos:  entry.start_pos,
                    end_pos:    mid_ray_pos
                })
            }

        } else {

            // One or both of the rays did not find a collision
            if entry.angle.end - entry.angle.start > min_step {
                // Cast a ray between the two points
                let mid_point   = (entry.angle.start + entry.angle.end) / 2.0;
                let mid_ray     = perform_ray_cast(center, mid_point, &cast_ray);
                let mid_ray_pos = mid_ray.as_ref().map(|(collision, _)| collision.position);

                // Divide into two pairs of ranges (process the earlier one first)
                stack.push(StackEntry {
                    angle:      mid_point..entry.angle.end,
                    start_pos:  mid_ray,
                    end_pos:    entry.end_pos
                });
                stack.push(StackEntry {
                    angle:      entry.angle.start..mid_point,
                    start_pos:  entry.start_pos,
                    end_pos:    mid_ray_pos
                })
            }
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
where
    Path:       BezierPathFactory<Point=Coord>,
    Coord:      Coordinate+Coordinate2D,
    RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
    RayFn:      Fn(Coord, Coord) -> RayList,
{
    // Trace where the ray casting algorithm indicates collisions with the specified center
    let collisions = trace_outline_convex(center, options, cast_ray);

    // Build a path using the LMS algorithm
    let curves = fit_curve::<Curve<Coord>>(&collisions.iter().map(|collision| collision.position).collect::<Vec<_>>(), options.fit_error);

    if let Some(curves) = curves {
        if !curves.is_empty() {
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
