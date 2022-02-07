use super::super::bezier::path::{GraphPath, GraphPathEdgeKind, PathDirection, PathLabel};
use super::super::bezier::{
    BezierCurve, BoundingBox, Bounds, Coordinate, Coordinate2D, NormalCurve,
};

use std::fmt::Write;

///
/// Writes out the graph path as an SVG string
///
pub fn graph_path_svg_string<P: Coordinate + Coordinate2D>(
    path: &GraphPath<P, PathLabel>,
    rays: Vec<(P, P)>,
) -> String {
    let mut result = String::new();

    let bounds = path
        .all_edges()
        .fold(Bounds::empty(), |a, b| a.union_bounds(b.bounding_box()));
    let offset = bounds.min();
    let scale = 1000.0 / (bounds.max() - bounds.min()).x();

    let mut index = 0;

    for kinds in vec![
        vec![
            GraphPathEdgeKind::Uncategorised,
            GraphPathEdgeKind::Visited,
            GraphPathEdgeKind::Interior,
        ],
        vec![GraphPathEdgeKind::Exterior],
    ] {
        for edge in path.all_edges() {
            if !kinds.contains(&edge.kind()) {
                continue;
            }

            let start_point = edge.start_point();
            let end_point = edge.end_point();
            let (cp1, cp2) = edge.control_points();

            writeln!(result, "<!-- {}: Curve::from_points(Coord2({}, {}), (Coord2({}, {}), Coord2({}, {})), Coord2({}, {})) -->", 
                index,
                start_point.x(), start_point.y(),
                cp1.x(), cp1.y(),
                cp2.x(), cp2.y(),
                end_point.x(), end_point.y()).unwrap();

            let start_point = (start_point - offset) * scale;
            let end_point = (end_point - offset) * scale;
            let cp1 = (cp1 - offset) * scale;
            let cp2 = (cp2 - offset) * scale;

            let kind = match edge.kind() {
                GraphPathEdgeKind::Uncategorised => "yellow",
                GraphPathEdgeKind::Visited => "red",
                GraphPathEdgeKind::Exterior => "blue",
                GraphPathEdgeKind::Interior => "green",
            };

            writeln!(result, "<path d=\"M {} {} C {} {}, {} {}, {} {}\" fill=\"transparent\" stroke-width=\"1\" stroke=\"{}\" />",
                start_point.x(), start_point.y(),
                cp1.x(), cp1.y(),
                cp2.x(), cp2.y(),
                end_point.x(), end_point.y(),
                kind).unwrap();
            writeln!(
                result,
                "<circle cx=\"{}\" cy=\"{}\" r=\"1.0\" fill=\"transparent\" stroke=\"magenta\" />",
                end_point.x(),
                end_point.y()
            )
            .unwrap();

            writeln!(
                result,
                "<text style=\"font-size: 8pt\" dx=\"{}\" dy=\"{}\">{} &lt;- {} - {}</text>",
                end_point.x() + 4.0,
                end_point.y() + 8.0,
                edge.end_point_index(),
                edge.start_point_index(),
                index
            )
            .unwrap();

            index += 1;
        }
    }

    for (p1, p2) in rays {
        writeln!(
            result,
            "<!-- Ray (Coord2({}, {}), Coord2({}, {})) -->",
            p1.x(),
            p1.y(),
            p2.x(),
            p2.y()
        )
        .unwrap();
        let collisions = path.ray_collisions(&(p1, p2));
        write!(result, "<!-- {} collisions -->", collisions.len()).unwrap();

        let p1 = (p1 - offset) * scale;
        let p2 = (p2 - offset) * scale;

        let point_offset = p2 - p1;
        let p1 = p1 - (point_offset * 1000.0);
        let p2 = p2 + (point_offset * 1000.0);

        writeln!(
            result,
            "<path d=\"M {} {} L {} {}\" fill=\"transparent\" stroke-width=\"1\" stroke=\"red\" />",
            p1.x(),
            p1.y(),
            p2.x(),
            p2.y()
        )
        .unwrap();

        let ray_direction = p2 - p1;
        let mut collision_count = 0;
        let mut collision_num = 0;

        for (collision, curve_t, _line_t, pos) in collisions {
            // Determine which direction the ray is crossing
            let edge = collision.edge();
            let PathLabel(path_number, direction) = path.edge_label(edge);
            let normal = path.get_edge(edge).normal_at_pos(curve_t);

            let side = ray_direction.dot(&normal).signum() as i32;
            let side = match direction {
                PathDirection::Clockwise => side,
                PathDirection::Anticlockwise => -side,
            };

            // Update the collision count
            collision_count += side;
            collision_num += 1;

            let pos = (pos - offset) * scale;

            let edge = path.get_edge(edge);
            let start_point = edge.start_point();
            let end_point = edge.end_point();
            let (cp1, cp2) = edge.control_points();

            writeln!(result, "<!-- Collision {} ({}): Curve::from_points(Coord2({}, {}), (Coord2({}, {}), Coord2({}, {})), Coord2({}, {})) -->", 
                collision_num, path_number,
                start_point.x(), start_point.y(),
                cp1.x(), cp1.y(),
                cp2.x(), cp2.y(),
                end_point.x(), end_point.y()).unwrap();

            let start_point = (start_point - offset) * scale;
            let end_point = (end_point - offset) * scale;
            let cp1 = (cp1 - offset) * scale;
            let cp2 = (cp2 - offset) * scale;

            writeln!(result, "<path d=\"M {} {} C {} {}, {} {}, {} {}\" fill=\"transparent\" stroke-width=\"1\" stroke=\"cyan\" />",
                start_point.x(), start_point.y(),
                cp1.x(), cp1.y(),
                cp2.x(), cp2.y(),
                end_point.x(), end_point.y()).unwrap();

            writeln!(
                result,
                "<circle cx=\"{}\" cy=\"{}\" r=\"1.0\" fill=\"transparent\" stroke=\"red\" />",
                pos.x(),
                pos.y()
            )
            .unwrap();
            writeln!(
                result,
                "<text style=\"font-size: 6pt\" dx=\"{}\" dy=\"{}\">{}: C{} ({})</text>",
                pos.x() + 2.0,
                pos.y() + 3.0,
                collision_num,
                collision_count,
                side
            )
            .unwrap();
        }
    }

    result
}
