use flo_curves::*;
use flo_curves::bezier::path::*;

///
/// A deterministic stand-in for a random number generator, so this test does
/// not need a dependency and always builds the same path.
///
struct Jitter(u64);

impl Jitter {
    fn next(&mut self) -> f64 {
        // xorshift64
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;

        ((x >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

///
/// Builds a tall, very narrow closed path: the vertices climb in y while their
/// x values jitter inside a band a few hundredths wide, so they arrive in no
/// particular x order.
///
/// That is the shape that matters for the ordering used by `exterior_paths()`:
/// some pairs of vertices are closer together in x than the tolerance and some
/// are further apart, and they are not encountered in sorted order.
///
fn narrow_jittered_spike(num_points: usize, x_band: f64) -> SimpleBezierPath {
    let mut jitter  = Jitter(0x2545_F491_4F6C_DD1D);
    let mut builder = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0));

    for idx in 0..num_points {
        let x = jitter.next() * x_band;
        let y = (idx as f64) * 0.5 + 1.0;

        builder = builder.line_to(Coord2(x, y));
    }

    // Close the path back around the outside
    builder = builder.line_to(Coord2(x_band + 5.0, (num_points as f64) * 0.5 + 1.0));
    builder = builder.line_to(Coord2(x_band + 5.0, 0.0));
    builder = builder.line_to(Coord2(0.0, 0.0));

    builder.build()
}

///
/// `exterior_paths()` orders the points of the graph before walking them, and
/// compares their x values with a tolerance: points within 0.01 of each other
/// are treated as sharing an x and ordered by y instead.
///
/// A tolerance comparison is not transitive. `a` and `b` can be within the
/// tolerance, and `b` and `c` within it, while `a` and `c` are not -- so the
/// ordering can report `a < b`, `b < c` and `c < a` at the same time. That is
/// not a valid strict weak ordering, and `sort_by` is documented as being
/// allowed to panic when it detects one, which current Rust versions do:
/// "user-provided comparison function does not correctly implement a total
/// order".
///
/// Every path arithmetic operation ends in `exterior_paths()`, so any of them
/// can hit this on artwork with many closely spaced points.
///
#[test]
fn add_paths_with_many_near_aligned_points() {
    let spike = narrow_jittered_spike(120, 0.05);
    let other = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(-1.0, -1.0))
        .line_to(Coord2(2.0, -1.0))
        .line_to(Coord2(2.0, 70.0))
        .line_to(Coord2(-1.0, 70.0))
        .line_to(Coord2(-1.0, -1.0))
        .build();

    let combined = path_add::<SimpleBezierPath>(&vec![spike], &vec![other], 0.01);

    assert!(!combined.is_empty());
}

///
/// The same path, subtracted rather than added: `path_sub` reaches the same
/// ordering code.
///
#[test]
fn subtract_paths_with_many_near_aligned_points() {
    let spike = narrow_jittered_spike(120, 0.05);
    let other = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(-1.0, 10.0))
        .line_to(Coord2(2.0, 10.0))
        .line_to(Coord2(2.0, 40.0))
        .line_to(Coord2(-1.0, 40.0))
        .line_to(Coord2(-1.0, 10.0))
        .build();

    let combined = path_sub::<SimpleBezierPath>(&vec![spike], &vec![other], 0.01);

    assert!(!combined.is_empty());
}
