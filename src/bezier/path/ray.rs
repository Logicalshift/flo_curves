use super::graph_path::*;
use super::super::curve::*;
use super::super::normal::*;
use super::super::intersection::*;
use super::super::super::geo::*;
use super::super::super::line::*;
use super::super::super::consts::*;

use smallvec::*;
use std::cmp::Ordering;

///
/// Represents a path that can be accessed by the ray collision algorithm
///
pub (crate) trait RayPath {
    type Point: Coordinate+Coordinate2D;
    type Curve: BezierCurve<Point=Self::Point>;

    ///
    /// Returns the number of points in this RayPath
    ///
    fn num_points(&self) -> usize;

    ///
    /// Returns the number of edges attached to a particular point
    ///
    fn num_edges(&self, point_idx: usize) -> usize;

    ///
    /// Returns references to the edges that arrive at the specified point
    ///
    fn reverse_edges_for_point(&self, point_idx: usize) -> SmallVec<[GraphEdgeRef; 8]>;

    ///
    /// Returns references to the edges that leave the specified point
    ///
    fn edges_for_point(&self, point_idx: usize) -> SmallVec<[GraphEdgeRef; 8]>;

    ///
    /// Maps an edge ref to an edge
    ///
    fn get_edge(&self, edge: GraphEdgeRef) -> Self::Curve;

    ///
    /// Returns the edge following the specified one
    ///
    fn get_next_edge(&self, edge: GraphEdgeRef) -> (GraphEdgeRef, Self::Curve);

    ///
    /// Returns the position of the point with the specified index
    ///
    fn point_position(&self, point: usize) -> Self::Point;

    ///
    /// Retrieves the start point of an edge
    ///
    fn edge_start_point_idx(&self, edge: GraphEdgeRef) -> usize;

    ///
    /// Retrieves the end point of an edge
    ///
    fn edge_end_point_idx(&self, edge: GraphEdgeRef) -> usize;

    ///
    /// Retrieves the index of the edge following the specified edge 
    /// (the edge start from the end point index that continues the path the edge is a part of)
    ///
    fn edge_following_edge_idx(&self, edge: GraphEdgeRef) -> usize;
}

///
/// Returns true if a curve is collinear given the set of coefficients for a ray
///
#[inline]
fn curve_is_collinear<Edge: BezierCurve>(edge: &Edge, (a, b, c): (f64, f64, f64)) -> bool
where Edge::Point: Coordinate+Coordinate2D {
    // Fetch the points of the curve
    let start_point = edge.start_point();
    let end_point   = edge.end_point();
    let (cp1, cp2)  = edge.control_points();

    // The curve is collinear if all of the points lie on the ray
    if (start_point.x()*a + start_point.y()*b + c).abs() < SMALL_DISTANCE
    && (end_point.x()*a + end_point.y()*b + c).abs() < SMALL_DISTANCE
    && (cp1.x()*a + cp1.y()*b + c).abs() < SMALL_DISTANCE
    && (cp2.x()*a + cp2.y()*b + c).abs() < SMALL_DISTANCE {
        true
    } else {
        false
    }
}

#[derive(PartialEq)]
enum RayCanIntersect {
    WrongSide,
    Collinear,
    CrossesRay
}

///
/// Given the coefficients of a ray, returns whether or not an edge can intersect it
///
fn ray_can_intersect<Edge: BezierCurve>(edge: &Edge, (a, b, c): (f64, f64, f64)) -> RayCanIntersect
where Edge::Point: Coordinate+Coordinate2D {
    // Fetch the points of the curve
    let start_point = edge.start_point();
    let end_point   = edge.end_point();
    let (cp1, cp2)  = edge.control_points();

    // Calculate distances to each of the points
    let start_distance  = a*start_point.x() + b*start_point.y() + c;
    let cp1_distance    = a*cp1.x() + b*cp1.y() + c;
    let cp2_distance    = a*cp2.x() + b*cp2.y() + c;
    let end_distance    = a*end_point.x()+ b*end_point.y() + c;

    // The sign of the distances indicate which side they're on
    let side            = start_distance.signum() + end_distance.signum() + cp1_distance.signum() + cp2_distance.signum();

    if start_distance.abs() < SMALL_DISTANCE && end_distance.abs() < SMALL_DISTANCE && cp1_distance.abs() < SMALL_DISTANCE && cp2_distance.abs() < SMALL_DISTANCE {
        // If all the distances are small enough, this section is collinear
        RayCanIntersect::Collinear
    } else if side < -3.99 || side > 3.99 {
        // If the side sums to 4, all points are on the same side
        RayCanIntersect::WrongSide
    } else {
        // Otherwise, the ray can intersect this line
        RayCanIntersect::CrossesRay
    }
}

///
/// Given a list of points, returns the edges that cross the line given by the specified set of coefficients
///
fn crossing_edges<Path: RayPath>(path: &Path, (a, b, c): (f64, f64, f64), points: Vec<usize>) -> Vec<GraphEdgeRef> {
    let mut crossing_edges = vec![];

    for point_idx in points.into_iter() {
        for incoming_ref in path.reverse_edges_for_point(point_idx) {
            // Get the incoming edge going in the right direction
            let incoming_ref    = incoming_ref.reversed();
            let incoming        = path.get_edge(incoming_ref);

            // Ignore collinear incoming edges
            if curve_is_collinear(&incoming, (a, b, c)) {
                continue;
            }

            // Fetch the leaving edge for the incoming edge
            let following_ref   = path.edge_following_edge_idx(incoming_ref);
            let mut leaving_ref = GraphEdgeRef { start_idx: point_idx, edge_idx: following_ref, reverse: false };
            let mut leaving     = path.get_edge(leaving_ref);

            // Follow the path until we complete a loop or find a leaving edge that's not collinear
            while curve_is_collinear(&leaving, (a, b, c)) {
                let (next_ref, next_edge) = path.get_next_edge(leaving_ref);

                leaving_ref = next_ref;
                leaving     = next_edge;

                if path.edge_start_point_idx(leaving_ref) == point_idx {
                    // Found a loop that was entirely collinear
                    // (Provided that the following edges always form a closed path this should always be reached, which is currently always true for the means we have to create a graph path)
                    break;
                }
            }

            // If it's not colinear, add to the set of crossing edges
            if !curve_is_collinear(&leaving, (a, b, c)) {
                let incoming_cp2    = incoming.control_points().1;
                let leaving_cp1     = leaving.control_points().0;

                let incoming_side   = a*incoming_cp2.x() + b*incoming_cp2.y() + c;
                let leaving_side    = a*leaving_cp1.x() + b*leaving_cp1.y() + c;

                if incoming_side.signum() != leaving_side.signum() {
                    // Control points are on different sides of the line, so this is a crossing edge
                    crossing_edges.push(leaving_ref);
                }
            }
        }
    }

    crossing_edges
}

///
/// Performs a basic search for collisions, returning them grouped into two sets.
/// 
/// The first set is crossing collisions. These are places where the ray met and edge at an angle and crossed it.
/// The second set is collinear collisions. These occur on straight edges that follow the same path as the ray.
///
#[inline(never)]
fn crossing_and_collinear_collisions<Path: RayPath, L: Line>(path: &Path, ray: &L) -> (SmallVec<[(GraphEdgeRef, f64, f64, Path::Point); 32]>, SmallVec<[(GraphEdgeRef, f64, f64, Path::Point); 8]>)
where   Path::Point:    Coordinate+Coordinate2D,
        L:              Line<Point=Path::Point> {
    let mut raw_collisions                                  = smallvec![];

    // If there are multiple collinear sections grouped together, these give them each a common identifier
    let mut section_with_point: Option<Vec<Option<usize>>>  = None;
    let mut collinear_sections: SmallVec<[Vec<_>; 8]>       = smallvec![];

    // The coefficients are used to determine if a particular edge can collide with the curve and if it's collinear or not
    let ray_coeffs = ray.coefficients();

    for point_idx in 0..(path.num_points()) {
        for edge_idx in 0..(path.num_edges(point_idx)) {
            let edge_ref    = GraphEdgeRef { start_idx: point_idx, edge_idx: edge_idx, reverse: false };
            let edge        = path.get_edge(edge_ref);

            let intersection_type = ray_can_intersect(&edge, ray_coeffs);

            match intersection_type {
                RayCanIntersect::CrossesRay => {
                    // This edge may intersect the ray
                    for (curve_t, line_t, collide_pos) in curve_intersects_ray(&edge, ray) {
                        // Store in the list of raw collisions
                        raw_collisions.push((edge_ref, curve_t, line_t, collide_pos));
                    }
                }

                RayCanIntersect::Collinear => {
                    // There are usually no collinear collisions, so only allocate our array if we find some
                    let section_with_point = section_with_point.get_or_insert_with(|| vec![None; path.num_points()]);

                    // This edge is collinear with the ray
                    let start_idx   = path.edge_start_point_idx(edge_ref);
                    let end_idx     = path.edge_end_point_idx(edge_ref);

                    if let Some(start_section) = section_with_point[start_idx] {
                        if let Some(_end_section) = section_with_point[end_idx] {
                            // Already seen an edge between these points
                        } else {
                            // end_idx is new
                            collinear_sections[start_section].push(end_idx);
                        }
                    } else if let Some(end_section) = section_with_point[end_idx] {
                        // start_idx is new
                        collinear_sections[end_section].push(start_idx);
                    } else {
                        // New section
                        let new_section = collinear_sections.len();
                        collinear_sections.push(vec![start_idx, end_idx]);
                        section_with_point[start_idx]   = Some(new_section);
                        section_with_point[end_idx]     = Some(new_section);
                    }
                }

                RayCanIntersect::WrongSide => { 
                    // Ray does not intersect the curve
                }
            }
        }
    }

    // Collect any collinear collisions into a vec
    let collinear_collisions = collinear_sections
        .into_iter()
        .flat_map(move |colinear_edge_points| crossing_edges(path, ray_coeffs, colinear_edge_points)
                .into_iter()
                .map(move |crossing_edge| {
                    let point   = path.edge_start_point_idx(crossing_edge);
                    let point   = path.point_position(point);
                    let line_t  = ray.pos_for_point(&point);

                    (crossing_edge, 0.0, line_t, point)
                }))
        .collect();

    (raw_collisions, collinear_collisions)
}

///
/// Given a list of collisions, removes any that are at the end just before a collinear section
///
#[inline]
fn remove_collisions_before_or_after_collinear_section<'a, Path: RayPath, L: Line, Collisions: 'a+IntoIterator<Item=(GraphEdgeRef, f64, f64, Path::Point)>>(path: &'a Path, ray: &L, collisions: Collisions) -> impl 'a+Iterator<Item=(GraphEdgeRef, f64, f64, Path::Point)>
where   Path::Point:    Coordinate+Coordinate2D,
        L:              Line<Point=Path::Point> {
    let ray_coeffs = ray.coefficients();

    collisions.into_iter()
        .filter(move |(collision, curve_t, _line_t, position)| {
            if *curve_t > 0.9 {
                let end_point_idx   = path.edge_end_point_idx(*collision);
                let end_point       = path.point_position(end_point_idx);

                // If any following edge is collinear, remove this collision
                if position.is_near_to(&end_point, CLOSE_DISTANCE) && path.edges_for_point(end_point_idx).into_iter().map(|edge| path.get_edge(edge)).any(|next| curve_is_collinear(&next, ray_coeffs)) {
                    false
                } else {
                    true
                }
            } else if *curve_t < 0.1 {
                let start_point_idx = path.edge_start_point_idx(*collision);
                let start_point     = path.point_position(start_point_idx);

                // If any preceding edge is collinear, remove this collision
                if position.is_near_to(&start_point, CLOSE_DISTANCE) && path.reverse_edges_for_point(start_point_idx).into_iter().map(|edge| path.get_edge(edge)).any(|previous| curve_is_collinear(&previous, ray_coeffs)) {
                    // Collisions crossing collinear sections are taken care of during the collinear collision phase
                    false
                } else {
                    true
                }
            } else {
                // Not at the end of a curve
                true
            }
        })
}

///
/// Given a list of collisions, finds any that are on a collinear line and moves them to the end of the collinear section
/// 
/// Collinear edges have the property that a ray collides with them on all points. We treat these as a collision at the start
/// of the following section, as this is the point where the line could enter or leave a shape.
///
#[inline]
fn move_collinear_collisions_to_end<'a, Path: RayPath, L: Line, Collisions: 'a+IntoIterator<Item=(GraphEdgeRef, f64, f64, Path::Point)>>(path: &'a Path, ray: &L, collisions: Collisions) -> impl 'a+Iterator<Item=(GraphEdgeRef, f64, f64, Path::Point)> 
where   Path::Point:    Coordinate+Coordinate2D,
        L:              Line<Point=Path::Point> {
    let ray_coeffs = ray.coefficients();

    collisions.into_iter()
        .map(move |(collision, curve_t, line_t, position)| {
            let edge = path.get_edge(collision);
            if curve_is_collinear(&edge, ray_coeffs) {
                let mut edge_ref    = collision;
                let mut edge;

                // Skip over collinear sections (they have 0 width from the point of view of the ray)
                loop {
                    let (next_edge_ref, next_edge) = path.get_next_edge(edge_ref);
                    edge_ref    = next_edge_ref;
                    edge        = next_edge;
                    if !curve_is_collinear(&edge, ray_coeffs) {
                        break;
                    }
                }

                let position = edge.start_point();
                (edge_ref, 0.0, line_t, position)
            } else {
                (collision, curve_t, line_t, position)
            }
        })
}

///
/// Returns true if the collision is at the start of the specified edge
///
#[inline]
fn collision_is_at_start<Path: RayPath>(path: &Path, edge: &GraphEdgeRef, curve_t: f64, position: &Path::Point) -> bool {
    if curve_t > 0.1 {
        false
    } else {
        let start_point = path.point_position(edge.start_idx);
        start_point.is_near_to(position, SMALL_DISTANCE)
    }
}

///
/// Returns true if the collision is at the end of the specified edge
///
#[inline]
fn collision_is_at_end<Path: RayPath>(path: &Path, edge: &GraphEdgeRef, curve_t: f64, position: &Path::Point) -> bool {
    if curve_t < 0.9 {
        false
    } else {
        let next_point_idx  = path.edge_end_point_idx(*edge);
        let end_point       = path.point_position(next_point_idx);
        end_point.is_near_to(position, SMALL_DISTANCE)
    }
}

///
/// Returns true if the 
///
#[inline]
fn edges_are_glancing<Path: RayPath>(path: &Path, ray: (f64, f64, f64), previous_edge: &GraphEdgeRef, following_edge: &GraphEdgeRef) -> bool {
    // Fetch the actual edges and take the ray apart
    let following_edge  = path.get_edge(*following_edge);
    let previous_edge   = path.get_edge(*previous_edge);
    let (a, b, c)       = ray;

    // A glancing collision has control points on the same side of the ray
    let cp_in           = previous_edge.control_points().1;
    let cp_out          = following_edge.control_points().0;

    let side_in         = cp_in.x()*a + cp_in.y()*b + c;
    let side_out        = cp_out.x()*a + cp_out.y()*b + c;

    let side_in         = if side_in.abs() < 0.001 { 0.0 } else { side_in.signum() };
    let side_out        = if side_out.abs() < 0.001 { 0.0 } else { side_out.signum() };

    // A glancing collision has both edges on the same side of the ray
    side_in == side_out
}

///
/// Removes extra collisions found near vertices
/// 
/// When a ray crosses at a vertex it will generate a collision at the end of one edge and the beginning 
/// of another. If this occurs, we should remove one of those collisions. As we use numerical methods to
/// solve for the collision point, it's possible to see only the 'end' or the 'start' collision.
/// 
/// It's possible for a ray to hit a vertex and not actually enter the shape. These are 'glancing' 
/// collisions. A glancing collision is one that generates exactly one collision at a corner without 
/// actually crossing into the shape (or which happens to hit a curve exactly on a tangent).
/// 
/// Corner collisions are found by looking for collisions at the start of an edge (we assume that the 
/// filtering in `move_collisions_at_end_to_beginning` has been applied) and checking if the following edge
/// crosses the array. If it does not, it's probably a glancing collision.
/// 
/// Tangent collisions are found just by looking at the tangent of the curve at the point of collision: if
/// it's collinear with the ray, then the ray presumably does not cross the edge.
/// 
/// As we use numerical methods to find line/curve collisions, it's possible for errors to result in a ray
/// that hits a corner and the other edge, so we finish up by filtering for this condition.
/// 
/// We need to filter these both in the same place as the choice of filter depends on whether or not a
/// particular collision is a glancing collision or a crossing collision.
///
fn filter_collisions_near_vertices<'a, Path: RayPath, L: Line, Collisions: 'a+IntoIterator<Item=(GraphEdgeRef, f64, f64, Path::Point)>>(path: &'a Path, ray: &'a L, collisions: Collisions) -> impl 'a+Iterator<Item=(GraphEdgeRef, f64, f64, Path::Point)>
where L: Line<Point=Path::Point> {
    let (a, b, c)           = ray.coefficients();
    let mut visited_start   = None;

    collisions.into_iter()
        .filter_map(move |(edge, curve_t, line_t, position)| {
            // This only applies to collisions at the end or start of an edge
            let is_at_start = collision_is_at_start(path, &edge, curve_t, &position);
            let is_at_end   = !is_at_start && collision_is_at_end(path, &edge, curve_t, &position);

            if is_at_start || is_at_end {
                // Collision might be crossing or glancing: get the two edges on the collision
                let (preceding_edge, following_edge) = if is_at_start {
                    let previous_edge   = path.reverse_edges_for_point(edge.start_idx)
                        .into_iter()
                        .map(|previous_edge| previous_edge.reversed())
                        .filter(|previous_edge| path.edge_following_edge_idx(*previous_edge) == edge.edge_idx)
                        .nth(0)
                        .expect("Previous edge for a collision at start");

                    (previous_edge, edge)
                } else {
                    let (next_edge, _) = path.get_next_edge(edge);

                    (edge, next_edge)
                };

                if edges_are_glancing(path, (a, b, c), &preceding_edge, &following_edge) {
                
                    // Ray hits close to a vertex between two edges that both face away from it (ie, may be a glancing collision)
                    // There must also be a glancing collision on the 'other' edge (we can afford this expensive check as glancing collisions are rare)
                    let both_glancing = if is_at_start {
                        // Must be a point close to the end of the preceding edge too
                        let edge            = path.get_edge(preceding_edge);
                        let collisions      = curve_intersects_ray(&edge, ray);

                        collisions.into_iter().any(|(curve_t, _line_t, position)| collision_is_at_end(path, &preceding_edge, curve_t, &position))
                    } else {
                        // Must be a point close to the start of the following edge too
                        let edge            = path.get_edge(following_edge);
                        let collisions      = curve_intersects_ray(&edge, ray);

                        collisions.into_iter().any(|(curve_t, _line_t, position)| collision_is_at_start(path, &following_edge, curve_t, &position))
                    };

                    if both_glancing {
                        // Remove both sides of a glancing collision
                        None
                    } else {
                        // Ray only gets close to the end of one of the edges: must be a crossing collision
                        Some((edge, curve_t, line_t, position))
                    }
                
                } else {

                    // Ray crosses exactly on the vertex: report it exactly once (as the beginning of the curve)
                    let visited_start = visited_start.get_or_insert_with(|| vec![None; path.num_points()]);

                    // At the start of the curve
                    let was_visited = visited_start[following_edge.start_idx]
                        .as_ref()
                        .map(|collisions: &SmallVec<[_; 2]>| collisions.contains(&following_edge.edge_idx))
                        .unwrap_or(false);

                    if !was_visited {
                        visited_start[following_edge.start_idx].get_or_insert_with(|| smallvec![]).push(following_edge.edge_idx);
                    }

                    if !was_visited {
                        Some((following_edge, 0.0, line_t, position))
                    } else {
                        None
                    }

                }
            } else {
                // In the middle: not glancing or crossing
                Some((edge, curve_t, line_t, position))
            }
        })
}

///
/// Removes any collision that manages to hit an edge exactly on a tangent
///
#[inline]
fn remove_tangent_collisions<'a, Path: RayPath, L: Line, Collisions: 'a+IntoIterator<Item=(GraphEdgeRef, f64, f64, Path::Point)>>(path: &'a Path, ray: &'a L, collisions: Collisions) -> impl 'a+Iterator<Item=(GraphEdgeRef, f64, f64, Path::Point)>
where L: Line<Point=Path::Point> {
    let ray_vector  = (ray.point_at_pos(1.0) - ray.point_at_pos(0.0)).to_unit_vector();

    collisions.into_iter()
        .filter(move |(edge, curve_t, _line_t, _position)| {
            // Check if we've hit a tangent
            let edge            = path.get_edge(*edge);

            // Get the curve tangent at this position
            let tangent         = edge.tangent_at_pos(*curve_t).to_unit_vector();

            // Test if it's going the same way as the ray
            let dot_product     = ray_vector.dot(&tangent);
            let dot_product_mag = dot_product.abs() - 1.0;

            // Dot product of two unit vectors will be 1.0 or -1.0 for a tangent collision
            if dot_product_mag > -0.00000001 && dot_product_mag < 0.00000001 {
                false
            } else {
                true
            }
        })
}

///
/// Finds any collision that occurred too close to an intersection and flags it as such
///
#[inline]
fn flag_collisions_at_intersections<'a, Path: RayPath, Collisions: 'a+IntoIterator<Item=(GraphEdgeRef, f64, f64, Path::Point)>>(path: &'a Path, collisions: Collisions) -> impl 'a+Iterator<Item=(GraphRayCollision, f64, f64, Path::Point)>
where   Path::Point: Coordinate+Coordinate2D {
    collisions
        .into_iter()
        .map(move |(collision, curve_t, line_t, position)| {
            if curve_t <= 0.000 {
                // Might be at an intersection (close to the start of the curve)
                if path.num_edges(collision.start_idx) > 1 {
                    // Intersection
                    (GraphRayCollision::Intersection(collision), curve_t, line_t, position)
                } else {
                    // Edge with only a single following point
                    (GraphRayCollision::SingleEdge(collision), curve_t, line_t, position)
                }
            } else {
                // Not at an intersection
                (GraphRayCollision::SingleEdge(collision), curve_t, line_t, position)
            }
        })
}

///
/// Finds all collisions between a ray and this path
/// 
pub (crate) fn ray_collisions<Path: RayPath, L: Line>(path: &Path, ray: &L) -> Vec<(GraphRayCollision, f64, f64, Path::Point)>
where   Path::Point:    Coordinate+Coordinate2D,
        L:              Line<Point=Path::Point> {
    let (p1, p2)        = ray.points();
    let ray_direction   = p2 - p1;

    // Raw collisions
    let (crossing_collisions, collinear_collisions) = crossing_and_collinear_collisions(path, ray);
    let collinear_collisions    = collinear_collisions.into_iter();
    let crossing_collisions     = crossing_collisions.into_iter();
    let crossing_collisions     = remove_collisions_before_or_after_collinear_section(path, ray, crossing_collisions);

    // Chain them together
    let collisions = collinear_collisions.chain(crossing_collisions);

    // Filter for accuracy
    let collisions = move_collinear_collisions_to_end(path, ray, collisions);
    let collisions = filter_collisions_near_vertices(path, ray, collisions);
    let collisions = remove_tangent_collisions(path, ray, collisions);
    let collisions = flag_collisions_at_intersections(path, collisions);

    // Convert to a vec and sort by ray position
    let mut collisions = collisions.collect::<Vec<_>>();

    collisions.sort_by(|(edge_a, curve_t_a, line_t_a, pos_a), (edge_b, curve_t_b, line_t_b, pos_b)| {
        // If the collision occurs at the same point on the line (within SMALL_DISTANCE), we need to order by edge priority. Otherwise, order by where collisions occur along the ray
        let dx  = pos_a.x() - pos_b.x();
        let dy  = pos_a.y() - pos_b.y();

        if dx.abs() > SMALL_DISTANCE || dy.abs() > SMALL_DISTANCE {
            // Order by position on the ray
            line_t_a.partial_cmp(line_t_b).unwrap_or(Ordering::Equal)
        } else {
            // Position on the line is the same (stabilise ordering by checking the edges)
            let edge_a = edge_a.edge();
            let edge_b = edge_b.edge();

            let result = edge_a.start_idx.cmp(&edge_b.start_idx);
            if result != Ordering::Equal {
                // Different start points
                result
            } else {
                // Check if these are the same edge or not
                let edge_order              = edge_a.edge_idx.cmp(&edge_b.edge_idx);

                // Ordering is reversed depending on the direction of the edge relative to the line
                // To produce a consistent ordering, we rely on edges from newer paths being added later in the list (having a higher edge_idx)
                // The ordering here is used with `set_edge_kinds_by_ray_casting()` to generate consistent results when paths overlap
                // TODO: it would be better to use label ordering here to get a more consistent result. This relies on the order that edges are added to the list
                let (earlier_edge, edge_t)  = match edge_order {
                    Ordering::Greater   => (edge_b, *curve_t_b),
                    Ordering::Less      => (edge_a, *curve_t_a),
                    Ordering::Equal     => { return Ordering::Equal; }
                };
                let earlier_edge            = path.get_edge(earlier_edge);
                let earlier_normal          = earlier_edge.normal_at_pos(edge_t);
                let earlier_direction       = ray_direction.dot(&earlier_normal);

                // TODO: reverse earlier_direction based if edge_a and edge_b are from shapes moving in different directions

                if earlier_direction < 0.0 {
                    edge_order.reverse()
                } else {
                    edge_order
                }
            }
        }
    });

    collisions
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::path::*;
    use super::super::path_builder::*;
    use super::super::graph_path::test::*;

    #[test]
    fn raw_donut_collisions() {
        let donut = donut();
        let donut = &donut;

        let raw_collisions = crossing_and_collinear_collisions(&donut, &(Coord2(7.000584357101389, 8.342524209216537), Coord2(6.941479643691172, 8.441210096108172))).0.into_iter();
        println!("{:?}", raw_collisions.collect::<Vec<_>>());

        // assert!(false);
    }

    #[test]
    fn collinear_collision_along_convex_edge_produces_no_collisions() {
        // Just one rectangle
        let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
            .line_to(Coord2(5.0, 1.0))
            .line_to(Coord2(5.0, 5.0))
            .line_to(Coord2(1.0, 5.0))
            .line_to(Coord2(1.0, 1.0))
            .build();

        // Collide along the vertical seam of this graph
        let gp = GraphPath::from_path(&rectangle1, ());
        let gp = &gp;

        let collisions = crossing_and_collinear_collisions(&gp, &(Coord2(5.0, 0.0), Coord2(5.0, 5.0))).1;
        assert!(collisions.len() == 0);
    }

    #[test]
    fn raw_collision_along_convex_edge_produces_no_collisions() {
        // Just one rectangle
        let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
            .line_to(Coord2(5.0, 1.0))
            .line_to(Coord2(5.0, 5.0))
            .line_to(Coord2(1.0, 5.0))
            .line_to(Coord2(1.0, 1.0))
            .build();

        // Collide along the vertical seam of this graph
        let gp = GraphPath::from_path(&rectangle1, ());
        let gp = &gp;

        let collisions = crossing_and_collinear_collisions(&gp, &(Coord2(5.0, 0.0), Coord2(5.0, 5.0))).0.into_iter();
        let collisions = remove_collisions_before_or_after_collinear_section(&gp, &(Coord2(5.0, 0.0), Coord2(5.0, 5.0)), collisions);
        let collisions = collisions.collect::<Vec<_>>();

        assert!(collisions.len() == 0);
    }

    #[test]
    fn collinear_collision_along_concave_edge_produces_single_collision() {
        let concave_shape = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
            .line_to(Coord2(5.0, 1.0))
            .line_to(Coord2(5.0, 5.0))
            .line_to(Coord2(6.0, 7.0))
            .line_to(Coord2(3.0, 7.0))
            .line_to(Coord2(1.0, 5.0))
            .line_to(Coord2(1.0, 1.0))
            .build();

        // Collide along the vertical seam of this graph
        let gp  = GraphPath::from_path(&concave_shape, ());
        let gp  = &gp;
        let ray = (Coord2(5.0, 0.0), Coord2(5.0, 5.0));

        let collisions = crossing_and_collinear_collisions(&gp, &ray).1;

        assert!(collisions.len() == 1);
    }

    #[test]
    fn raw_collision_along_concave_edge_produces_single_collision() {
        let concave_shape = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
            .line_to(Coord2(5.0, 1.0))
            .line_to(Coord2(5.0, 5.0))
            .line_to(Coord2(6.0, 7.0))
            .line_to(Coord2(3.0, 7.0))
            .line_to(Coord2(1.0, 5.0))
            .line_to(Coord2(1.0, 1.0))
            .build();

        // Collide along the vertical seam of this graph
        let gp  = GraphPath::from_path(&concave_shape, ());
        let gp  = &gp;
        let ray = (Coord2(5.0, 0.0), Coord2(5.0, 5.0));

        let collisions = crossing_and_collinear_collisions(&gp, &ray).0.into_iter();
        let collisions = remove_collisions_before_or_after_collinear_section(&gp, &(Coord2(5.0, 0.0), Coord2(5.0, 5.0)), collisions);
        let collisions = collisions.collect::<Vec<_>>();

        assert!(collisions.len() == 1);
    }

    #[test]
    fn concave_collision_breakdown() {
        let concave_shape = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
            .line_to(Coord2(5.0, 1.0))
            .line_to(Coord2(5.0, 5.0))
            .line_to(Coord2(6.0, 7.0))
            .line_to(Coord2(3.0, 7.0))
            .line_to(Coord2(1.0, 5.0))
            .line_to(Coord2(1.0, 1.0))
            .build();

        // Collide along the vertical seam of this graph
        let gp  = GraphPath::from_path(&concave_shape, ());
        let gp  = &gp;
        let ray = (Coord2(5.0, 0.0), Coord2(5.0, 5.0));

        // Raw collisions
        let (normal_collisions, collinear_collisions) = crossing_and_collinear_collisions(&gp, &ray);
        let normal_collisions       = remove_collisions_before_or_after_collinear_section(&gp, &ray, normal_collisions).collect::<Vec<_>>();

        assert!(collinear_collisions.len() == 1);
        assert!(normal_collisions.len() == 1);

        // Chain them together
        let collisions = collinear_collisions.into_iter().chain(normal_collisions.into_iter()).collect::<Vec<_>>();
        assert!(collisions.len() == 2);

        // Filter for accuracy
        let collisions = move_collinear_collisions_to_end(&gp, &ray, collisions).collect::<Vec<_>>();
        assert!(collisions.len() == 2);
        let collisions = filter_collisions_near_vertices(&gp, &ray, collisions).collect::<Vec<_>>();
        assert!(collisions.len() == 2);
        let collisions = flag_collisions_at_intersections(&gp, collisions).collect::<Vec<_>>();
        assert!(collisions.len() == 2);
    }

    #[test]
    fn interior_point_produces_four_collisions() {
        let with_interior_point = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
            .line_to(Coord2(5.0, 1.0))
            .line_to(Coord2(5.0, 5.0))
            .line_to(Coord2(2.0, 2.0))
            .line_to(Coord2(4.0, 2.0))
            .line_to(Coord2(1.0, 5.0))
            .line_to(Coord2(1.0, 1.0))
            .build();

        let mut with_interior_point = GraphPath::from_path(&with_interior_point, ());
        with_interior_point.self_collide(0.01);
        let with_interior_point     = &with_interior_point;

        let ray         = (Coord2(0.0, 3.0), Coord2(1.0, 3.0));
        let collisions  = crossing_and_collinear_collisions(&with_interior_point, &ray).0;

        println!("{:?}", with_interior_point);
        println!("{:?}", collisions);

        assert!(collisions.len() == 4);

        // Filter for accuracy
        let collisions = move_collinear_collisions_to_end(&with_interior_point, &ray, collisions).collect::<Vec<_>>();
        assert!(collisions.len() == 4);
        let collisions = filter_collisions_near_vertices(&with_interior_point, &ray, collisions).collect::<Vec<_>>();
        println!("{:?}", collisions);
        assert!(collisions.len() == 4);
        let collisions = flag_collisions_at_intersections(&with_interior_point, collisions).collect::<Vec<_>>();
        assert!(collisions.len() == 4);
    }
}
