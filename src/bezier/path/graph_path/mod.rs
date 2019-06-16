use super::path::*;
use super::super::curve::*;
use super::super::super::geo::*;
use super::super::super::consts::*;

use smallvec::*;

use std::fmt;
use std::cell::*;

mod edge;
mod edge_ref;
mod ray_collision;
mod path_collision;

#[cfg(test)] pub (crate) mod test;

pub use self::edge::*;
pub use self::edge_ref::*;
pub use self::ray_collision::*;
pub use self::path_collision::*;

/// Maximum number of edges to traverse when 'healing' gaps found in an external path
const MAX_HEAL_DEPTH: usize = 3;

///
/// Kind of a graph path edge
/// 
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GraphPathEdgeKind {
    /// An edge that hasn't been categorised yet
    Uncategorised,

    /// An edge that is uncategorised but has been visited
    Visited,

    /// An exterior edge
    /// 
    /// These edges represent a transition between the inside and the outside of the path
    Exterior, 

    /// An interior edge
    /// 
    /// These edges are on the inside of the path
    Interior
}

///
/// Reference to a graph edge
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GraphEdgeRef {
    /// The index of the point this edge starts from
    pub (crate) start_idx: usize,

    /// The index of the edge within the point
    pub (crate) edge_idx: usize,

    /// True if this reference is for the reverse of this edge
    pub (crate) reverse: bool
}

///
/// Enum representing an edge in a graph path
/// 
#[derive(Clone, Debug)]
struct GraphPathEdge<Point, Label> {
    /// The label attached to this edge
    label: Label,

    /// The ID of the edge following this one on the target point
    following_edge_idx: usize,

    /// The kind of this edge
    kind: GraphPathEdgeKind,

    /// Position of the first control point
    cp1: Point,

    /// Position of the second control point
    cp2: Point,

    /// The index of the target point
    end_idx: usize,

    /// The bounding box of this edge, if it has been calculated
    bbox: RefCell<Option<(Point, Point)>>
}

///
/// Struct representing a point in a graph path
///
#[derive(Clone, Debug)]
struct GraphPathPoint<Point, Label> {
    /// The position of this point
    position: Point,

    /// The edges attached to this point
    forward_edges: SmallVec<[GraphPathEdge<Point, Label>; 2]>,

    /// The points with edges connecting to this point
    connected_from: SmallVec<[usize; 2]>
}

impl<Point, Label> GraphPathPoint<Point, Label> {
    ///
    /// Creates a new graph path point
    ///
    fn new(position: Point, forward_edges: SmallVec<[GraphPathEdge<Point, Label>; 2]>, connected_from: SmallVec<[usize; 2]>) -> GraphPathPoint<Point, Label> {
        GraphPathPoint { position, forward_edges, connected_from }
    }
}

///
/// A graph path is a path where each point can have more than one connected edge. Edges are categorized
/// into interior and exterior edges depending on if they are on the outside or the inside of the combined
/// shape.
/// 
#[derive(Clone)]
pub struct GraphPath<Point, Label> {
    /// The points in this graph and their edges. Each 'point' here consists of two control points and an end point
    points: Vec<GraphPathPoint<Point, Label>>,

    /// The index to assign to the next path added to this path
    next_path_index: usize
}

impl<Point: Coordinate, Label> Geo for GraphPath<Point, Label> {
    type Point = Point;
}

impl<Point: Coordinate+Coordinate2D, Label: Copy> GraphPath<Point, Label> {
    ///
    /// Creates a new graph path with no points
    ///
    pub fn new() -> GraphPath<Point, Label> {
        GraphPath {
            points:             vec![],
            next_path_index:    0
        }
    }

    ///
    /// Creates a graph path from a bezier path
    /// 
    pub fn from_path<P: BezierPath<Point=Point>>(path: &P, label: Label) -> GraphPath<Point, Label> {
        // All edges are exterior for a single path
        let mut points = vec![];

        // Push the start point (with an open path)
        let start_point = path.start_point();
        points.push(GraphPathPoint::new(start_point, smallvec![], smallvec![]));

        // We'll add edges to the previous point
        let mut last_point_pos  = start_point;
        let mut last_point_idx  = 0;
        let mut next_point_idx  = 1;

        // Iterate through the points in the path
        for (cp1, cp2, end_point) in path.points() {
            // Ignore points that are too close to the last point
            if end_point.is_near_to(&last_point_pos, CLOSE_DISTANCE) {
                if cp1.is_near_to(&last_point_pos, CLOSE_DISTANCE) && cp2.is_near_to(&cp1, CLOSE_DISTANCE) {
                    continue;
                }
            }

            // Push the points
            points.push(GraphPathPoint::new(end_point, smallvec![], smallvec![]));

            // Add an edge from the last point to the next point
            points[last_point_idx].forward_edges.push(GraphPathEdge::new(GraphPathEdgeKind::Uncategorised, (cp1, cp2), next_point_idx, label, 0));

            // Update the last/next pooints
            last_point_idx  += 1;
            next_point_idx  += 1;
            last_point_pos  = end_point;
        }

        // Close the path
        if last_point_idx > 0 {
            // Graph actually has some edges
            if start_point.distance_to(&points[last_point_idx].position) < CLOSE_DISTANCE {
                // Remove the last point (we're replacing it with an edge back to the start)
                points.pop();
                last_point_idx -= 1;

                // Change the edge to point back to the start
                points[last_point_idx].forward_edges[0].end_idx = 0;
            } else {
                // Need to draw a line to the last point (as there is always a single following edge, the following edge index is always 0 here)
                let close_vector    = points[last_point_idx].position - start_point;
                let cp1             = close_vector * 0.33 + start_point;
                let cp2             = close_vector * 0.66 + start_point;

                points[last_point_idx].forward_edges.push(GraphPathEdge::new(GraphPathEdgeKind::Uncategorised, (cp1, cp2), 0, label, 0));
            }
        } else {
            // Just a start point and no edges: remove the start point as it doesn't really make sense
            points.pop();
        }

        // Create the graph path from the points
        let mut path = GraphPath {
            points:             points,
            next_path_index:    1
        };
        path.recalculate_reverse_connections();
        path
    }

    ///
    /// Creates a new graph path by merging (not colliding) a set of paths with their labels
    ///
    pub fn from_merged_paths<'a, P: 'a+BezierPath<Point=Point>, PathIter: IntoIterator<Item=(&'a P, Label)>>(paths: PathIter) -> GraphPath<Point, Label> {
        // Create an empty path
        let mut merged_path = GraphPath::new();

        // Merge each path in turn
        for (path, label) in paths {
            let path    = GraphPath::from_path(path, label);
            merged_path = merged_path.merge(path);
        }

        merged_path
    }

    ///
    /// Recomputes the list of items that have connections to each point
    ///
    fn recalculate_reverse_connections(&mut self) {
        // Reset the list of connections to be empty
        for point_idx in 0..(self.points.len()) {
            self.points[point_idx].connected_from.clear();
        }

        // Add a reverse connection for every edge
        for point_idx in 0..(self.points.len()) {
            for edge_idx in 0..(self.points[point_idx].forward_edges.len()) {
                let end_idx = self.points[point_idx].forward_edges[edge_idx].end_idx;
                self.points[end_idx].connected_from.push(point_idx);
            }
        }

        // Sort and deduplicate them
        for point_idx in 0..(self.points.len()) {
            self.points[point_idx].connected_from.sort();
            self.points[point_idx].connected_from.dedup();
        }
    }

    ///
    /// Returns the number of points in this graph. Points are numbered from 0 to this value.
    /// 
    #[inline]
    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    ///
    /// Returns an iterator of all edges in this graph
    ///
    #[inline]
    pub fn all_edges<'a>(&'a self) -> impl 'a+Iterator<Item=GraphEdge<'a, Point, Label>> {
        (0..(self.points.len()))
            .into_iter()
            .flat_map(move |point_num| self.edges_for_point(point_num))
    }

    ///
    /// Returns an iterator of all the edges in this graph, as references
    ///
    #[inline]
    pub fn all_edge_refs<'a>(&'a self) -> impl 'a+Iterator<Item=GraphEdgeRef> {
        (0..(self.points.len()))
            .into_iter()
            .flat_map(move |point_idx| (0..(self.points[point_idx].forward_edges.len()))
                .into_iter()
                .map(move |edge_idx| GraphEdgeRef {
                    start_idx:  point_idx,
                    edge_idx:   edge_idx,
                    reverse:    false
                }))
    }

    ///
    /// Returns an iterator of the edges that leave a particular point
    /// 
    /// Edges are directional: this will provide the edges that leave the supplied point
    ///
    #[inline]
    pub fn edges_for_point<'a>(&'a self, point_num: usize) -> impl 'a+Iterator<Item=GraphEdge<'a, Point, Label>> {
        (0..(self.points[point_num].forward_edges.len()))
            .into_iter()
            .map(move |edge_idx| GraphEdge::new(self, GraphEdgeRef { start_idx: point_num, edge_idx: edge_idx, reverse: false }))
    }

    ///
    /// Returns the edge refs for a particular point
    ///
    pub fn edge_refs_for_point(&self, point_num: usize) -> impl Iterator<Item=GraphEdgeRef> {
        (0..(self.points[point_num].forward_edges.len()))
            .into_iter()
            .map(move |edge_idx| GraphEdgeRef { start_idx: point_num, edge_idx: edge_idx, reverse: false })
    }

    ///
    /// Returns the position of a particular point
    ///
    #[inline]
    pub fn point_position(&self, point_num: usize) -> Point {
        self.points[point_num].position.clone()
    }

    ///
    /// Returns an iterator of the edges that arrive at a particular point
    /// 
    /// Edges are directional: this will provide the edges that connect to the supplied point
    ///
    pub fn reverse_edges_for_point<'a>(&'a self, point_num: usize) -> impl 'a+Iterator<Item=GraphEdge<'a, Point, Label>> {
        // Fetch the points that connect to this point
        self.points[point_num].connected_from
            .iter()
            .flat_map(move |connected_from| {
                let connected_from = *connected_from;

                // Any edge that connects to the current point, in reverse
                (0..(self.points[connected_from].forward_edges.len()))
                    .into_iter()
                    .filter_map(move |edge_idx| {
                        if self.points[connected_from].forward_edges[edge_idx].end_idx == point_num {
                            Some(GraphEdgeRef { start_idx: connected_from, edge_idx: edge_idx, reverse: true })
                        } else {
                            None
                        }
                    })
            })
            .map(move |edge_ref| GraphEdge::new(self, edge_ref))
    }

    ///
    /// Merges in another path
    /// 
    /// This adds the edges in the new path to this path without considering if they are internal or external 
    ///
    pub fn merge(self, merge_path: GraphPath<Point, Label>) -> GraphPath<Point, Label> {
        // Copy the points from this graph
        let mut new_points  = self.points;
        let next_path_idx   = self.next_path_index;

        // Add in points from the merge path
        let offset          = new_points.len();
        new_points.extend(merge_path.points.into_iter()
            .map(|mut point| {
                // Update the offsets in the edges
                for mut edge in &mut point.forward_edges {
                    edge.end_idx            += offset;
                }

                for previous_point in &mut point.connected_from {
                    *previous_point += offset;
                }

                // Generate the new edge
                point
            }));

        // Combined path
        GraphPath {
            points:             new_points,
            next_path_index:    next_path_idx + merge_path.next_path_index
        }
    }

    ///
    /// Returns true if the specified edge is very short (starts and ends at the same point and does not cover a significant amount of ground)
    ///
    fn edge_is_very_short(&self, edge_ref: GraphEdgeRef) -> bool {
        let edge        = &self.points[edge_ref.start_idx].forward_edges[edge_ref.edge_idx];

        if edge_ref.start_idx == edge.end_idx {
            // Find the points on this edge
            let start_point = &self.points[edge_ref.start_idx].position;
            let cp1         = &edge.cp1;
            let cp2         = &edge.cp2;
            let end_point   = &self.points[edge.end_idx].position;

            // If all the points are close to each other, then this is a short edge
            start_point.is_near_to(end_point, CLOSE_DISTANCE)
                && start_point.is_near_to(cp1, CLOSE_DISTANCE)
                && cp1.is_near_to(cp2, CLOSE_DISTANCE)
                && cp2.is_near_to(end_point, CLOSE_DISTANCE)
        } else {
            false
        }
    }

    ///
    /// Removes an edge by updating the previous edge to point at its next edge
    /// 
    /// Control points are not updated so the shape will be distorted if the removed edge is very long
    ///
    fn remove_edge(&mut self, edge_ref: GraphEdgeRef) {
        // Edge consistency is preserved provided that the edges are already consistent
        self.check_following_edge_consistency();

        // Find the next edge
        let next_point_idx      = self.points[edge_ref.start_idx].forward_edges[edge_ref.edge_idx].end_idx;
        let next_edge_idx       = self.points[edge_ref.start_idx].forward_edges[edge_ref.edge_idx].following_edge_idx;

        // Edge shouldn't just loop around to itself
        debug_assert!(next_point_idx != edge_ref.start_idx || next_edge_idx != edge_ref.edge_idx);

        // ... and the preceding edge (by searching all of the connected points)
        let previous_edge_ref   = self.points[edge_ref.start_idx].connected_from
            .iter()
            .map(|point_idx| { let point_idx = *point_idx; self.points[point_idx].forward_edges.iter().enumerate().map(move |(edge_idx, edge)| (point_idx, edge_idx, edge)) })
            .flatten()
            .filter_map(|(point_idx, edge_idx, edge)| {
                if edge.end_idx == edge_ref.start_idx && edge.following_edge_idx == edge_ref.edge_idx {
                    Some(GraphEdgeRef { start_idx: point_idx, edge_idx: edge_idx, reverse: false })
                } else {
                    None
                }
            })
            .nth(0);

        debug_assert!(previous_edge_ref.is_some());

        if let Some(previous_edge_ref) = previous_edge_ref {
            debug_assert!(self.points[previous_edge_ref.start_idx].forward_edges[previous_edge_ref.edge_idx].end_idx == edge_ref.start_idx);
            debug_assert!(self.points[previous_edge_ref.start_idx].forward_edges[previous_edge_ref.edge_idx].following_edge_idx == edge_ref.edge_idx);

            // Reconnect the previous edge to the next edge
            self.points[previous_edge_ref.start_idx].forward_edges[previous_edge_ref.edge_idx].end_idx              = next_point_idx;
            self.points[previous_edge_ref.start_idx].forward_edges[previous_edge_ref.edge_idx].following_edge_idx   = next_edge_idx;

            // Remove the old edge from the list
            self.points[edge_ref.start_idx].forward_edges.remove(edge_ref.edge_idx);

            // For all the connected points, update the following edge refs
            let mut still_connected = false;

            // Double connected from would result in us updating a edge twice, so ensure any duplicates are removed here
            self.points[edge_ref.start_idx].connected_from.sort();
            self.points[edge_ref.start_idx].connected_from.dedup();

            for connected_point_idx in self.points[edge_ref.start_idx].connected_from.clone() {
                for edge_idx in 0..(self.points[connected_point_idx].forward_edges.len()) {
                    let connected_edge = &mut self.points[connected_point_idx].forward_edges[edge_idx];

                    // Only interested in edges on the point we just changed
                    if connected_edge.end_idx != edge_ref.start_idx {
                        continue;
                    }

                    // We should have eliminated the edge we're deleting when we updated the edge above
                    debug_assert!(connected_edge.following_edge_idx != edge_ref.edge_idx);

                    // Update the following edge if it was affected by the deletion
                    if connected_edge.following_edge_idx > edge_ref.edge_idx {
                        connected_edge.following_edge_idx -= 1;
                    }

                    // If there's another edge ending at the original point, then we're still connected
                    if connected_edge.end_idx == edge_ref.start_idx {
                        still_connected = true;
                    }
                }
            }

            // If the two points are not still connected, remove the previous point from the connected list
            if !still_connected {
                self.points[edge_ref.start_idx].connected_from.retain(|point_idx| *point_idx != edge_ref.start_idx);
            }

            // Edges should be consistent again
            self.check_following_edge_consistency();
        }
    }

    ///
    /// Removes an edge considered to be very short
    ///
    fn remove_very_short_edge(&mut self, edge_ref: GraphEdgeRef) {
        let mut edge_ref = edge_ref;

        self.check_following_edge_consistency();

        // Replace the content of this edge with the content of the following edge
        let next_point_idx  = self.points[edge_ref.start_idx].forward_edges[edge_ref.edge_idx].end_idx;
        let next_edge_idx   = self.points[edge_ref.start_idx].forward_edges[edge_ref.edge_idx].following_edge_idx;

        debug_assert!(next_point_idx != edge_ref.start_idx || next_edge_idx != edge_ref.edge_idx);

        // Replace the short edge with the next edge (if edges are very short, we don't need to adjust control points here)
        self.points[edge_ref.start_idx].forward_edges[edge_ref.edge_idx] = self.points[next_point_idx].forward_edges[next_edge_idx].clone();

        // Remove the next edge (we just replaced it)
        self.points[next_point_idx].forward_edges.remove(next_edge_idx);

        if edge_ref.start_idx == next_point_idx && edge_ref.edge_idx > next_edge_idx {
            // Loop that starts and ends at the same point
            edge_ref.edge_idx -= 1;
        }

        // Any other edge coming into next_point_idx needs to be updated
        let mut new_connected_from = smallvec![edge_ref.start_idx];

        // Iterate over all the previous items
        for previous_idx in self.points[next_point_idx].connected_from.clone() {
            for mut edge in self.points[previous_idx].forward_edges.iter_mut().filter(|edge| edge.end_idx == next_point_idx) {
                // Is connected from this edge
                new_connected_from.push(previous_idx);

                // Update the edge pointer
                debug_assert!(edge.following_edge_idx != next_edge_idx);
                if edge.following_edge_idx > next_edge_idx {
                    edge.following_edge_idx -= 1;
                }
            }
        }

        self.check_following_edge_consistency();

        // Remove duplicates
        new_connected_from.sort();
        new_connected_from.dedup();

        // Update the 'connected from' items for the new next point
        self.points[next_point_idx].connected_from = new_connected_from;

        // Also update the connected from list for the next point
        let new_next_point_idx      = self.points[edge_ref.start_idx].forward_edges[edge_ref.edge_idx].following_edge_idx;
        let mut next_connected_from = self.points[new_next_point_idx].connected_from.clone();

        // Is now connected from this edge
        next_connected_from.push(edge_ref.start_idx);
        next_connected_from.sort();
        next_connected_from.dedup();

        // Make sure that all the points are still connected that are supposed to be
        next_connected_from
            .retain(|previous_point_idx| self.points[*previous_point_idx].forward_edges
                .iter()
                .any(|next_edge| next_edge.end_idx == new_next_point_idx));

        // Finish by updating the connected from list
        self.points[new_next_point_idx].connected_from = next_connected_from;
    }

    ///
    /// Removes any edges that appear to be 'very short' from this graph
    /// 
    /// 'Very short' edges are edges that start and end at the same point and have control points very close to the start position
    ///
    fn remove_all_very_short_edges(&mut self) {
        for point_idx in 0..(self.points.len()) {
            let mut edge_idx = 0;
            while edge_idx < self.points[point_idx].forward_edges.len() {
                // Remove this edge if it's very short
                let edge_ref = GraphEdgeRef { start_idx: point_idx, edge_idx: edge_idx, reverse: false };
                if self.edge_is_very_short(edge_ref) {
                    self.remove_edge(edge_ref);
                }

                // Next edge
                edge_idx += 1;
            }
        }
    }

    ///
    /// Collides this path against another, generating a merged path
    /// 
    /// Anywhere this graph intersects the second graph, a point with two edges will be generated. All edges will be left as
    /// interior or exterior depending on how they're set on the graph they originate from.
    /// 
    /// Working out the collision points is the first step to performing path arithmetic: the resulting graph can be altered
    /// to specify edge types - knowing if an edge is an interior or exterior edge makes it possible to tell the difference
    /// between a hole cut into a shape and an intersection.
    /// 
    pub fn collide(mut self, collide_path: GraphPath<Point, Label>, accuracy: f64) -> GraphPath<Point, Label> {
        // Generate a merged path with all of the edges
        let collision_offset    = self.points.len();
        self                    = self.merge(collide_path);

        // Search for collisions between our original path and the new one
        let total_points = self.points.len();
        self.detect_collisions(0..collision_offset, collision_offset..total_points, accuracy);

        // Return the result
        self
    }

    ///
    /// Rounds all of the points in this path to a particular accuracy level
    ///
    pub fn round(&mut self, accuracy: f64) {
        for point_idx in 0..(self.num_points()) {
            self.points[point_idx].position.round(accuracy);

            for edge_idx in 0..(self.points[point_idx].forward_edges.len()) {
                self.points[point_idx].forward_edges[edge_idx].cp1.round(accuracy);
                self.points[point_idx].forward_edges[edge_idx].cp2.round(accuracy);
            }
        }
    }

    ///
    /// Finds any collisions between existing points in the graph path
    ///
    pub fn self_collide(&mut self, accuracy: f64) {
        let total_points = self.points.len();
        self.detect_collisions(0..total_points, 0..total_points, accuracy);
    }

    ///
    /// Returns the GraphEdge for an edgeref
    ///
    #[inline]
    pub fn get_edge<'a>(&'a self, edge: GraphEdgeRef) -> GraphEdge<'a, Point, Label> {
        GraphEdge::new(self, edge)
    }

    ///
    /// Sets the kind of a single edge
    ///
    #[inline]
    pub fn set_edge_kind(&mut self, edge: GraphEdgeRef, new_type: GraphPathEdgeKind) {
        self.points[edge.start_idx].forward_edges[edge.edge_idx].kind = new_type;
    }

    ///
    /// Sets the label of a single edge
    ///
    #[inline]
    pub fn set_edge_label(&mut self, edge: GraphEdgeRef, new_label: Label) {
        self.points[edge.start_idx].forward_edges[edge.edge_idx].label = new_label;
    }

    ///
    /// Returns the type of the edge pointed to by an edgeref
    ///
    #[inline]
    pub fn edge_kind(&self, edge: GraphEdgeRef) -> GraphPathEdgeKind {
        self.points[edge.start_idx].forward_edges[edge.edge_idx].kind
    }

    ///
    /// Returns the label of the edge pointed to by an edgeref
    ///
    #[inline]
    pub fn edge_label(&self, edge: GraphEdgeRef) -> Label {
        self.points[edge.start_idx].forward_edges[edge.edge_idx].label
    }

    ///
    /// Sets the kind of an edge and any connected edge where there are no intersections (only one edge)
    ///
    pub fn set_edge_kind_connected(&mut self, edge: GraphEdgeRef, kind: GraphPathEdgeKind) {
        let mut current_edge    = edge;
        let mut visited         = vec![false; self.points.len()];

        // Move forward
        loop {
            // Set the kind of the current edge
            self.set_edge_kind(current_edge, kind);
            visited[current_edge.start_idx] = true;

            // Pick the next edge
            let end_idx = self.points[current_edge.start_idx].forward_edges[current_edge.edge_idx].end_idx;
            let edges   = &self.points[end_idx].forward_edges;

            if edges.len() != 1 {
                // At an intersection
                break;
            } else {
                // Move on
                current_edge = GraphEdgeRef {
                    start_idx:  end_idx,
                    edge_idx:   0,
                    reverse:    false
                }
            }

            // Also stop if we've followed the loop all the way around
            if visited[current_edge.start_idx] {
                break;
            }
        }

        // Move backwards
        current_edge = edge;
        loop {
            // Mark this point as visited
            visited[current_edge.start_idx] = true;

            if self.points[current_edge.start_idx].connected_from.len() != 1 {
                // There is more than one incoming edge
                break;
            } else {
                // There's a single preceding point (but maybe more than one edge)
                let current_point_idx   = current_edge.start_idx;
                let previous_point_idx  = self.points[current_edge.start_idx].connected_from[0];

                // Find the index of the preceding edge
                let mut previous_edges  = (0..(self.points[previous_point_idx].forward_edges.len()))
                    .into_iter()
                    .filter(|edge_idx| self.points[previous_point_idx].forward_edges[*edge_idx].end_idx == current_point_idx);

                let previous_edge_idx   = previous_edges.next().expect("Previous edge");
                if previous_edges.next().is_some() {
                    // There is more than one edge connecting these two points
                    break;
                }

                // Move on to the next edge
                current_edge = GraphEdgeRef {
                    start_idx:  previous_point_idx,
                    edge_idx:   previous_edge_idx,
                    reverse:    false
                };

                // Change its kind
                self.set_edge_kind(current_edge, kind);
            }

            // Also stop if we've followed the loop all the way around
            if visited[current_edge.start_idx] {
                break;
            }
        }
    }

    ///
    /// Returns true if the specified point has a single exterior edge attached to it
    ///
    fn has_single_exterior_edge(&self, point_idx: usize) -> bool {
        self.edges_for_point(point_idx)
            .chain(self.reverse_edges_for_point(point_idx))
            .filter(|edge| edge.kind() == GraphPathEdgeKind::Exterior)
            .count() == 1
    }

    ///
    /// Returns true if the specified edge has a gap (end point has no following exterior edge)
    ///
    fn edge_has_gap(&self, edge: GraphEdgeRef) -> bool {
        // Interior edges have no gaps
        if self.points[edge.start_idx].forward_edges[edge.edge_idx].kind != GraphPathEdgeKind::Exterior {
            false
        } else {
            // Get the end point index for this edge
            let (start_idx, end_idx) = if edge.reverse {
                (self.points[edge.start_idx].forward_edges[edge.edge_idx].end_idx, edge.start_idx)
            } else {
                (edge.start_idx, self.points[edge.start_idx].forward_edges[edge.edge_idx].end_idx)
            };

            // Result is true if there is no edge attached to the end point that is marked exterior (other than the edge leading back to the initial point)
            !self.edges_for_point(end_idx)
                .chain(self.reverse_edges_for_point(end_idx))
                .filter(|following_edge| following_edge.end_point_index() != start_idx)
                .any(|following_edge| following_edge.kind() == GraphPathEdgeKind::Exterior)
        }
    }

    ///
    /// Given an edge that ends in a gap, attempts to bridge the gap by finding a following edge that has no following exterior edges on
    /// its start point.
    ///
    fn heal_edge_with_gap(&mut self, point_idx: usize, edge_idx: usize, max_depth: usize) -> bool {
        // This is Dijsktra's algorithm again: we also use this for a similar purpose in exterior_paths
        let end_point_idx = self.points[point_idx].forward_edges[edge_idx].end_idx;

        // State of the algorithm
        let mut preceding_edge      = vec![None; self.points.len()];
        let mut points_to_process   = vec![(point_idx, end_point_idx)];
        let mut current_depth       = 0;
        let mut target_point_idx    = None;

        // Iterate until we hit the maximum depth
        while current_depth < max_depth && target_point_idx.is_none() {
            // Points found in this pass that need to be checked
            let mut next_points_to_process = vec![];

            // Process all the points found in the previous pass
            for (from_point_idx, next_point_idx) in points_to_process {
                // Stop once we find a point
                if target_point_idx.is_some() { break; }

                // Process all edges connected to this point
                for next_edge in self.edges_for_point(next_point_idx) /*.chain(self.reverse_edges_for_point(next_point_idx)) */ {
                    let edge_end_point_idx  = next_edge.end_point_index();
                    let next_edge_ref       = GraphEdgeRef::from(&next_edge);
                    let edge_start_idx      = next_edge.start_point_index();

                    // Don't go back the way we came
                    if edge_end_point_idx == from_point_idx { continue; }

                    // Don't revisit points we already have a trail for
                    if preceding_edge[edge_end_point_idx].is_some() { continue; }

                    // Ignore exterior edges (except exterior edges where edge_has_gap is true, which indicate we've crossed our gap)
                    let mut reversed_edge_ref = next_edge_ref;
                    reversed_edge_ref.reverse = !reversed_edge_ref.reverse;
                    if next_edge.kind() == GraphPathEdgeKind::Exterior && !self.edge_has_gap(reversed_edge_ref) { continue; }

                    // Add this as a preceding edge
                    preceding_edge[edge_end_point_idx] = Some(next_edge_ref);

                    // We've found a path across the gap if we find an exterior edge
                    if next_edge.kind() == GraphPathEdgeKind::Exterior {
                        // Set this as the target point
                        target_point_idx = Some(edge_end_point_idx);
                        break;
                    }

                    // Continue searching from this point
                    next_points_to_process.push((edge_start_idx, edge_end_point_idx));
                }
            }

            // Process any points we found in the next pass
            points_to_process = next_points_to_process;

            // Moved down a level in the graph
            current_depth += 1;
        }

        if let Some(target_point_idx) = target_point_idx {
            // Target_point represents the final point in the 
            let mut current_point_idx = target_point_idx;

            while current_point_idx != end_point_idx {
                let previous_edge_ref = preceding_edge[current_point_idx].expect("Previous point during gap healing");

                // Mark this edge as exterior
                self.points[previous_edge_ref.start_idx].forward_edges[previous_edge_ref.edge_idx].kind = GraphPathEdgeKind::Exterior;

                // Move to the previous point
                let previous_edge = self.get_edge(previous_edge_ref);
                current_point_idx = previous_edge.start_point_index();
            }

            true
        } else {
            // Failed to cross the gap
            false
        }
    }

    ///
    /// Finds any gaps in the edges marked as exterior and attempts to 'heal' them by finding a route to another
    /// part of the path with a missing edge
    /// 
    /// Returns true if all the gaps that were found were 'healed'
    ///
    pub fn heal_exterior_gaps(&mut self) -> bool {
        let mut all_healed = true;

        // Iterate over all the edges in this graph
        for point_idx in 0..(self.points.len()) {
            for edge_idx in 0..(self.points[point_idx].forward_edges.len()) {
                // If this edge has a gap...
                if self.edge_has_gap(GraphEdgeRef { start_idx: point_idx, edge_idx: edge_idx, reverse: false }) {
                    // ... try to heal it
                    if !self.heal_edge_with_gap(point_idx, edge_idx, MAX_HEAL_DEPTH) {
                        all_healed = false;
                    }
                }
            }
        }

        all_healed
    }

    ///
    /// Finds the exterior edges and turns them into a series of paths
    ///
    pub fn exterior_paths<POut: BezierPathFactory<Point=Point>>(&self) -> Vec<POut> {
        // List of paths returned by this function
        let mut exterior_paths = vec![];

        // Array of points visited on a path that we've added to the result
        let mut visited = vec![false; self.points.len()];

        let mut previous_point                      = vec![None; self.points.len()];
        let mut points_to_check: SmallVec<[_; 16]>  = smallvec![];

        for point_idx in 0..(self.points.len()) {
            // Ignore this point if we've already visited it as part of a path
            if visited[point_idx] {
                continue;
            }

            // Use Dijkstra's algorithm to search for the shortest path that returns to point_idx
            // This allows for loops or other constructs to exist within the edges, which can happen with sufficiently complicated arithmetic operations
            // The result will sometimes be incorrect for these situations.
            // (Ideally we'd try to find a path that visits some points multiple times when this happens)
            for p in previous_point.iter_mut() {
                *p = None;
            }

            points_to_check.clear();
            points_to_check.push((point_idx, point_idx));

            // Loop until we find a previous point for the initial point (indicating we've got a loop of points)
            while previous_point[point_idx].is_none() {
                if points_to_check.len() == 0 {
                    // Ran out of points to check to find a loop (there is no loop for this point)
                    break;
                }

                let mut next_points_to_check: SmallVec<[_; 16]> = smallvec![];

                // Check all of the points we found last time (ie, breadth-first search of the graph)
                for (previous_point_idx, current_point_idx) in points_to_check {
                    let mut edges = if current_point_idx == point_idx {
                        // For the first point, only search forward
                        self.reverse_edges_for_point(current_point_idx).collect::<SmallVec<[_; 8]>>()
                    } else {
                        // For all other points, search all edges
                        self.edges_for_point(current_point_idx)
                            .chain(self.reverse_edges_for_point(current_point_idx))
                            .collect::<SmallVec<[_; 8]>>()
                    };

                    // Only follow exterior edges...
                    if current_point_idx == point_idx || edges.iter().any(|edge| edge.kind() == GraphPathEdgeKind::Exterior && edge.end_point_index() != previous_point_idx) {
                        // ... unless the only exterior edge is the one we arrived on, in which case we'll follow interior edges to try to bridge gaps as a backup measure
                        edges.retain(|edge| edge.kind() == GraphPathEdgeKind::Exterior);
                    } else {
                        // Search for edges with a single following exterior edge
                        edges.retain(|edge| self.has_single_exterior_edge(edge.end_point_index()));
                    }

                    // Follow the edges for this point
                    for edge in edges {
                        // Find the point that this edge goes to
                        let next_point_idx = edge.end_point_index();

                        if previous_point[next_point_idx].is_some() {
                            // We've already visited this point
                            continue;
                        }

                        if next_point_idx == previous_point_idx {
                            // This edge is going backwards around the graph
                            continue;
                        }

                        // Record the current point as the previous point for the end point of this edge
                        previous_point[next_point_idx] = Some((current_point_idx, edge));

                        // Check the edges connected to this point next
                        next_points_to_check.push((current_point_idx, next_point_idx));
                    }
                }

                // Check the set of points we found during this run through the loop next time
                points_to_check = next_points_to_check;
            }

            // If we found a loop, generate a path
            if previous_point[point_idx].is_some() {
                let mut path_points     = vec![];
                let mut cur_point_idx   = point_idx;

                while let Some((last_point_idx, ref edge)) = previous_point[cur_point_idx] {
                    // Push to the path points (we're following the edges in reverse, so points are in reverse order)
                    let (cp1, cp2)  = edge.control_points();
                    let start_point = edge.start_point();

                    path_points.push((cp2, cp1, start_point));

                    // Mark this point as visited so we don't try to include it in a future path
                    visited[last_point_idx] = true;

                    // Move back along the path
                    cur_point_idx = last_point_idx;

                    if cur_point_idx == point_idx {
                        // Finished the loop
                        break;
                    }
                }

                // Start point of the path is the initial point we checked
                let start_point = self.points[point_idx].position.clone();

                let new_path    = POut::from_points(start_point, path_points);
                exterior_paths.push(new_path);
            }
        }

        // Return the set of exterior paths
        exterior_paths
    }
}

///
/// Represents an edge in a graph path
/// 
#[derive(Clone)]
pub struct GraphEdge<'a, Point: 'a, Label: 'a> {
    /// The graph that this point is for
    graph: &'a GraphPath<Point, Label>,

    /// A reference to the edge this point is for
    edge: GraphEdgeRef
}

impl<Point: Coordinate2D+Coordinate+fmt::Debug, Label: Copy> fmt::Debug for GraphPath<Point, Label> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for point_idx in 0..(self.points.len()) {
            write!(f, "\nPoint {:?}:", point_idx)?;

            for edge in self.edges_for_point(point_idx) {
                write!(f, "\n  {:?}", edge)?;
            }
        }

        Ok(())
    }
}
