use super::fill_convex::*;
use super::fill_settings::*;
use super::super::*;
use super::super::super::*;
use super::super::super::super::geo::*;

use std::f64;

///
/// Traces the outline of a complex area using ray-casting
///
/// While the convex version of this function can only trace the outline of a region as it can be reached by a single ray, this
/// concave version can trace outlines with edges that are not in 'line of sight' from the origin point. The extra work required
/// for this can be quite time-consuming, so an efficient ray-casting function is vital if the path is particularly complex.
///
/// The current version of the algorithm works by taking the result from a convex ray-cast and finding large gaps where no points
/// were detected, and filling them in by ray-casting from there. There are cases where the resulting path can overlap itself: after
/// fitting the curve, use `remove_interior_points` to generate a non-overlapping path.
///
pub fn trace_outline_concave<Coord, Item, RayList, RayFn>(center: Coord, options: &FillSettings, cast_ray: RayFn) -> Vec<RayCollision<Coord, Item>> 
where   Coord:      Coordinate+Coordinate2D,
        RayList:    IntoIterator<Item=RayCollision<Coord, Item>>,
        RayFn:      Fn(Coord, Coord) -> RayList {
    unimplemented!()
}
