use flo_curves::bezier::path::*;
use flo_curves::geo::*;

use std::fmt::Write;

pub fn svg_path_string<Path: BezierPath>(path: &Path) -> String
where
    Path::Point: Coordinate2D,
{
    let mut svg = String::new();

    write!(
        svg,
        "M {} {}",
        path.start_point().x(),
        path.start_point().y()
    )
    .unwrap();
    for (cp1, cp2, end) in path.points() {
        write!(
            svg,
            " C {} {}, {} {}, {} {}",
            cp1.x(),
            cp1.y(),
            cp2.x(),
            cp2.y(),
            end.x(),
            end.y()
        )
        .unwrap();
    }

    svg
}
