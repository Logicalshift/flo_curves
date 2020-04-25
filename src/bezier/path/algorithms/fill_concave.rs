use super::fill_convex::*;
use super::fill_settings::*;
use super::super::super::super::geo::*;

use std::f64;

///
/// Item returned from a ray cast intersection by the concave raycasting function (where we can hit an existing edge)
///
#[derive(Clone, Debug)]
enum ConcaveItem<Item> {
    /// Edge returned by the main raycasting function
    Edge(Item),

    /// Intersection with an edge detected in an earlier raycasting operation
    SelfIntersection(usize)
}

///
/// Represents a long edge that we want to raycast from
///
struct LongEdge<Coord> {
    start:          Coord,
    end:            Coord,
    edge_index:     (usize, usize),
    ray_collided:   bool
}

///
/// Retrieves the 'long' edges from a set of edges returned by a raycast tracing operation
///
fn find_long_edges<Coord, Item>(edges: &[RayCollision<Coord, Item>], edge_min_len_squared: f64) -> Vec<LongEdge<Coord>>
where Coord: Coordinate+Coordinate2D {
    // Find the edges where we need to cast extra rays
    let mut long_edges      = vec![];
    for edge_num in 0..edges.len() {
        // Get the length of this edge
        let last_edge               = if edge_num == 0 {
            edges.len() - 1
        } else {
            edge_num-1
        };

        let edge_offset             = edges[last_edge].position - edges[edge_num].position;
        let edge_distance_squared   = edge_offset.dot(&edge_offset);

        // Add to the list of long edges if it's long enough to need further ray-casting
        if edge_distance_squared >= edge_min_len_squared {
            long_edges.push(LongEdge { 
                start:          edges[last_edge].position.clone(),
                end:            edges[edge_num].position.clone(),
                edge_index:     (last_edge, edge_num),
                ray_collided:   false
            })
        }
    }

    long_edges
}

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
    let cast_ray                = &cast_ray;

    // The edge min length is the length of edge we need to see before we'll 'look around' a corner
    let edge_min_len            = options.step * 4.0;
    let edge_min_len_squared    = edge_min_len * edge_min_len;

    // Perform the initial convex ray-casting
    let mut edges = trace_outline_convex(center, options, cast_ray);

    // Stop if we found no collisions
    if edges.len() < 2 {
        return vec![];
    }

    // Find the edges where we need to cast extra rays
    let mut long_edges      = find_long_edges(&edges, edge_min_len_squared);

    // TODO: cast rays from each of the 'long' edges and update the edge list
    let mut long_edge_index = 0;
    while long_edge_index < long_edges.len() {
        // Fetch the next edge to cast from
        let next_edge = &long_edges[long_edge_index];

        // Skip edges where we've already self-intersected
        if !next_edge.ray_collided {
            // Pick the center point
            let center_point = (next_edge.start + next_edge.end) * 0.5;
            let offset      = next_edge.start - next_edge.end;

            // Find the angle of the next edge
            let line_angle  = offset.x().atan2(offset.y());

            // Perform raycasting over a 180 degree angle to get the next set of edges
            // TODO: plus collide with lines we've added
            let new_edges   = trace_outline_convex_partial(center_point, options, line_angle..(line_angle+f64::consts::PI), cast_ray);

            if new_edges.len() > 2 {
                // We ignore the first and last point as they will be the points along the existing edge (ie, will be the start and end points we already know)

                // Find new long edges in the new edges
                let mut new_long_edges  = find_long_edges(&new_edges[1..(new_edges.len()-1)], edge_min_len_squared);

                // Don't count the edge ending at point 0 (that's the edge we just came from)
                new_long_edges.retain(|edge| edge.edge_index.1 != 0);

                // Insert the new edges into the existing edge list (except the first and last which will be duplicates)
                let edge_index      = next_edge.edge_index.1;
                let num_new_edges   = new_edges.len()-2;
                edges.splice(edge_index..edge_index, new_edges.into_iter().skip(1).take(num_new_edges));

                // Update the remaining long edge indexes
                for update_idx in long_edge_index..long_edges.len() {
                    if long_edges[update_idx].edge_index.0 >= edge_index {
                        long_edges[update_idx].edge_index.0 += num_new_edges;
                    }

                    if long_edges[update_idx].edge_index.1 >= edge_index {
                        long_edges[update_idx].edge_index.1 += num_new_edges;
                    }
                }

                // Add the new long edges to the list
                for edge in new_long_edges.iter_mut() {
                    edge.edge_index.0 += edge_index;
                    edge.edge_index.1 += edge_index;
                }

                long_edges.splice((long_edge_index+1)..(long_edge_index+1), new_long_edges);
            }
        }

        // Check the next edge
        long_edge_index += 1;
    }

    // The edges we retrieved are the result
    edges
}
