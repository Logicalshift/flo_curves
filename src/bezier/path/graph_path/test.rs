use super::*;
use super::super::*;
use super::super::super::normal::*;
use super::super::super::super::arc::*;

pub (crate) fn donut() -> GraphPath<Coord2, ()> {
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle1   = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(9.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle2   = Circle::new(Coord2(9.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    let mut circle1     = GraphPath::from_path(&circle1, ());
    circle1             = circle1.merge(GraphPath::from_path(&inner_circle1, ()));
    let mut circle2     = GraphPath::from_path(&circle2, ());
    circle2             = circle2.merge(GraphPath::from_path(&inner_circle2, ()));

    circle1.collide(circle2, 0.1)
}

pub fn tricky_path1() -> SimpleBezierPath {
    BezierPathBuilder::<SimpleBezierPath>::start(Coord2(266.4305, 634.9583))
        .curve_to((Coord2(267.89352, 634.96545), Coord2(276.2691, 647.3115)), Coord2(283.95255, 660.0379))
        .curve_to((Coord2(287.94046, 666.35474), Coord2(291.91766, 672.60645)), Coord2(295.15033, 677.43414))
        .curve_to((Coord2(296.7672, 679.91516), Coord2(298.1211, 681.9124)), Coord2(299.32123, 683.47577))
        .curve_to((Coord2(299.95978, 684.32623), Coord2(300.40076, 684.9176)), Coord2(300.98044, 685.51074))
        .curve_to((Coord2(301.33307, 685.8545), Coord2(301.51462, 686.0718)), Coord2(301.92783, 686.3648))
        .curve_to((Coord2(302.63144, 686.6535), Coord2(302.6845, 686.9835)), Coord2(303.79065, 687.13))
        .curve_to((Coord2(308.23322, 698.75146), Coord2(314.235, 706.79364)), Coord2(320.5527, 711.571))
        .curve_to((Coord2(323.84628, 713.9084), Coord2(326.7522, 715.38696)), Coord2(329.93036, 715.9504))
        .curve_to((Coord2(333.10065, 716.4182), Coord2(336.06982, 716.2095)), Coord2(338.80997, 715.17615))
        .curve_to((Coord2(344.1068, 713.1569), Coord2(348.558, 708.8886)), Coord2(352.09903, 704.2416))
        .curve_to((Coord2(355.6339, 699.64606), Coord2(358.63943, 694.3838)), Coord2(361.0284, 690.2511))
        .curve_to((Coord2(352.29608, 691.48425), Coord2(348.7531, 697.58563)), Coord2(344.9467, 702.02875))
        .curve_to((Coord2(343.1644, 704.2118), Coord2(340.9616, 706.1748)), Coord2(338.98895, 707.4077))
        .curve_to((Coord2(337.17404, 708.7338), Coord2(334.93362, 709.2896)), Coord2(332.94815, 709.3193))
        .curve_to((Coord2(338.20477, 716.0944), Coord2(342.99326, 713.658)), Coord2(346.69864, 710.2048))
        .curve_to((Coord2(350.41446, 706.8076), Coord2(353.61026, 702.4266)), Coord2(356.28525, 698.20306))
        .curve_to((Coord2(358.8071, 690.86554), Coord2(368.403, 680.78076)), Coord2(364.57346, 683.4333))
        .curve_to((Coord2(370.74402, 683.10126), Coord2(380.93408, 677.46747)), Coord2(391.3346, 669.7194))
        .curve_to((Coord2(401.82745, 661.6356), Coord2(411.92975, 652.304)), Coord2(416.44824, 642.7813))
        .curve_to((Coord2(421.56387, 630.7548), Coord2(419.29, 605.44073)), Coord2(418.97845, 598.63885))
        .curve_to((Coord2(416.0324, 600.9351), Coord2(416.06793, 605.21173)), Coord2(415.80798, 610.2456))
        .curve_to((Coord2(418.3617, 603.8127), Coord2(419.7235, 595.5345)), Coord2(417.99966, 597.9464))
        .curve_to((Coord2(417.83536, 597.29565), Coord2(417.6163, 596.428)), Coord2(417.452, 595.7772))
        .curve_to((Coord2(415.13226, 598.33954), Coord2(417.1024, 601.5625)), Coord2(415.80798, 610.2456))
        .curve_to((Coord2(419.39615, 605.133), Coord2(419.15756, 600.892)), Coord2(418.97845, 598.63885))
        .curve_to((Coord2(415.9, 605.6454), Coord2(416.15115, 630.697)), Coord2(410.98987, 640.1752))
        .curve_to((Coord2(407.398, 647.65436), Coord2(397.31293, 657.55756)), Coord2(387.45657, 664.45013))
        .curve_to((Coord2(377.50784, 671.67847), Coord2(367.18683, 676.76263)), Coord2(364.60056, 676.3969))
        .curve_to((Coord2(356.0477, 679.03125), Coord2(358.2825, 685.37573)), Coord2(350.3949, 694.47205))
        .curve_to((Coord2(347.86517, 698.46545), Coord2(345.09418, 702.3025)), Coord2(342.02982, 705.0691))
        .curve_to((Coord2(338.955, 707.7797), Coord2(336.14987, 709.45294)), Coord2(332.94815, 709.3193))
        .curve_to((Coord2(336.5865, 716.2577), Coord2(339.58755, 714.99677)), Coord2(342.64694, 713.29364))
        .curve_to((Coord2(345.54865, 711.4972), Coord2(347.85297, 709.2183)), Coord2(350.22574, 706.551))
        .curve_to((Coord2(354.72943, 701.2933), Coord2(358.0882, 695.26)), Coord2(361.0284, 690.2511))
        .curve_to((Coord2(352.55414, 690.95703), Coord2(349.8117, 695.7842)), Coord2(346.5798, 700.0057))
        .curve_to((Coord2(343.354, 704.1756), Coord2(340.01953, 707.4518)), Coord2(336.43625, 708.6749))
        .curve_to((Coord2(334.73633, 709.2627), Coord2(332.9918, 709.5996)), Coord2(331.1653, 709.1589))
        .curve_to((Coord2(329.34668, 708.8136), Coord2(326.97275, 707.9294)), Coord2(324.69394, 706.071))
        .curve_to((Coord2(319.86685, 702.45667), Coord2(313.55374, 694.77545)), Coord2(307.1513, 682.14154))
        .curve_to((Coord2(305.31448, 680.437), Coord2(305.08902, 680.6507)), Coord2(305.46603, 680.73413))
        .curve_to((Coord2(305.55258, 680.8219), Coord2(305.35938, 680.745)), Coord2(305.29236, 680.7117))
        .curve_to((Coord2(305.03268, 680.5507), Coord2(304.45453, 680.05615)), Coord2(303.91962, 679.53674))
        .curve_to((Coord2(302.7728, 678.36035), Coord2(301.16226, 676.48175)), Coord2(299.40033, 674.3327))
        .curve_to((Coord2(295.8753, 669.90015), Coord2(291.43716, 663.8746)), Coord2(286.9764, 657.9508))
        .curve_to((Coord2(277.76248, 646.196), Coord2(269.10742, 634.2079)), Coord2(266.40128, 634.45917))
        .curve_to((Coord2(266.42087, 634.7936), Coord2(266.41122, 634.6289)), Coord2(266.4305, 634.9583))
        .build()
}

fn overlapping_rectangle() -> SimpleBezierPath {
    BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 3.0))
        .line_to(Coord2(7.0, 5.0))
        .line_to(Coord2(5.0, 7.0))
        .line_to(Coord2(3.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build()
}

fn looped_rectangle() -> SimpleBezierPath {
    BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))

        //.line_to(Coord2(2.0, 1.0))
        .line_to(Coord2(3.0, 1.0))
        .line_to(Coord2(3.0, 5.0))
        .line_to(Coord2(2.0, 5.0))
        .line_to(Coord2(2.0, 1.0))
        .line_to(Coord2(3.0, 1.0))

        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))

        //.line_to(Coord2(3.0, 5.0))
        .line_to(Coord2(2.0, 5.0))
        .line_to(Coord2(2.0, 1.0))
        .line_to(Coord2(3.0, 1.0))
        .line_to(Coord2(3.0, 5.0))
        .line_to(Coord2(2.0, 5.0))

        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build()
}

#[test]
fn ray_cast_with_tricky_path_after_self_collide() {
    let tricky      = tricky_path1();
    let mut tricky  = GraphPath::from_path(&tricky, ());

    tricky.self_collide(0.01);

    for edge in tricky.all_edges() {
        let target  = edge.point_at_pos(0.5);
        let normal  = edge.normal_at_pos(0.5);
        let ray     = (target, target+normal);

        let collisions = tricky.ray_collisions(&ray);

        // Should be an even number of collisions
        assert!((collisions.len()&1) == 0);
    }
}

#[test]
fn single_difficult_ray_cast_with_tricky_path_before_self_collide() {
    let tricky      = tricky_path1();
    let tricky      = GraphPath::from_path(&tricky, ());

    let ray         = (Coord2(344.7127586558301, 702.311674360346), Coord2(344.6914625870749, 702.2935114955856));
    let collisions  = tricky.ray_collisions(&ray);

    println!("{:?}", tricky);
    println!("{:?}", collisions);
    assert!((collisions.len()&1) == 0);
}

#[test]
fn single_difficult_ray_cast_with_tricky_path_after_self_collide() {
    let tricky      = tricky_path1();
    let mut tricky  = GraphPath::from_path(&tricky, ());

    tricky.self_collide(0.01);

    let ray         = (Coord2(344.7127586558301, 702.311674360346), Coord2(344.6914625870749, 702.2935114955856));
    let collisions  = tricky.ray_collisions(&ray);

    println!("{:?}", tricky);
    println!("{:?}", collisions);
    assert!((collisions.len()&1) == 0);
}

#[test]
fn overlapping_rectangle_ray_cast_after_self_collide() {
    let overlapping     = overlapping_rectangle();
    let mut overlapping = GraphPath::from_path(&overlapping, ());

    overlapping.self_collide(0.01);

    let ray         = (Coord2(3.0, 0.0), Coord2(3.0, 5.0));
    let collisions  = overlapping.ray_collisions(&ray);

    println!("{:?}", overlapping);
    println!("{:?}", collisions);
    assert!((collisions.len()&1) == 0);
}

#[test]
fn looped_rectangle_ray_cast_after_self_collide() {
    let looped     = looped_rectangle();
    let mut looped = GraphPath::from_path(&looped, ());

    looped.self_collide(0.01);
    println!("{:?}", looped);

    for edge in looped.all_edges() {
        let target  = edge.point_at_pos(0.5);
        let normal  = edge.normal_at_pos(0.5);
        let ray     = (target, target+normal);

        let collisions = looped.ray_collisions(&ray);

        // Should be an even number of collisions
        assert!((collisions.len()&1) == 0);
    }
}

#[test]
fn find_gaps() {
    let path            = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path  = GraphPath::from_path(&path, ());
    let edges           = (0..4).into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).nth(0).unwrap().into())
        .collect::<Vec<_>>();

    graph_path.set_edge_kind(edges[0], GraphPathEdgeKind::Exterior);
    graph_path.set_edge_kind(edges[2], GraphPathEdgeKind::Exterior);
    graph_path.set_edge_kind(edges[3], GraphPathEdgeKind::Exterior);

    // Edge 0,0 is followed by a gap
    assert!(graph_path.edge_has_gap(GraphEdgeRef { start_idx: 0, edge_idx: 0, reverse: false }));

    // Edge 1,0 is the gap
    assert!(!graph_path.edge_has_gap(GraphEdgeRef { start_idx: 1, edge_idx: 0, reverse: false }));

    // Edge 2,0 is preceded by the gap
    assert!(graph_path.edge_has_gap(GraphEdgeRef { start_idx: 2, edge_idx: 0, reverse: true }));
}
