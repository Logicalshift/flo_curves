use super::checks::*;

use flo_curves::geo::*;
use flo_curves::bezier::path::*;

///
/// Creates a permutation of a path, by rotating and optionally reversing the points
///
pub fn path_permutation(path: Vec<Coord2>, start_offset: usize, forward: bool) -> SimpleBezierPath {
    let mut result = BezierPathBuilder::start(path[start_offset]);

    for idx in 1..path.len() {
        let pos = if forward {
            (start_offset + idx) % path.len()
        } else {
            let idx = (path.len()) - idx;
            (start_offset + idx) % path.len()
        };

        result = result.line_to(path[pos]);
    }

    result = result.line_to(path[start_offset]);

    result.build()
}

#[test]
fn permutation_matches_original() {
    let path        = vec![Coord2(206.0, 391.0), Coord2(206.0, 63.0), Coord2(281.0, 66.0), Coord2(281.0, 320.0), Coord2(649.0, 320.0), Coord2(649.0, 63.0), Coord2(734.0, 63.0), Coord2(734.0, 391.0)];
    let base_path   = path_permutation(path.clone(), 0, true);

    for forward in [true, false] {
        for permutation in 0..path.len() {
            println!("Forward: {:?}, permutation: {}/{}", forward, permutation, path.len());

            let permuted = path_permutation(path.clone(), permutation, forward);

            assert!(path_has_points_in_order(permuted, base_path.1.iter().cloned().collect(), 0.1));
        }
    }
}