use flo_curves::geo::*;

use std::cmp::{Ordering};

#[test]
fn sweep_self_single_overlap() {
    let mut bounds = vec![
        Bounds::from_min_max(Coord2(100.0, 200.0), Coord2(200.0, 300.0)),
        Bounds::from_min_max(Coord2(150.0, 250.0), Coord2(250.0, 350.0)),
    ];
    bounds.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions = sweep_self(bounds.iter()).collect::<Vec<_>>();

    assert!(collisions.len() == 1);
}

#[test]
fn sweep_self_double_overlap() {
    let mut bounds = vec![
        Bounds::from_min_max(Coord2(100.0, 200.0), Coord2(200.0, 300.0)),
        Bounds::from_min_max(Coord2(150.0, 250.0), Coord2(250.0, 350.0)),
        Bounds::from_min_max(Coord2(220.0, 330.0), Coord2(350.0, 450.0)),
    ];
    bounds.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions = sweep_self(bounds.iter()).collect::<Vec<_>>();

    assert!(collisions.len() == 2);
}

#[test]
fn sweep_self_triple_overlap() {
    let mut bounds = vec![
        Bounds::from_min_max(Coord2(100.0, 200.0), Coord2(200.0, 300.0)),
        Bounds::from_min_max(Coord2(150.0, 250.0), Coord2(250.0, 350.0)),
        Bounds::from_min_max(Coord2(190.0, 290.0), Coord2(290.0, 390.0)),
    ];
    bounds.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions = sweep_self(bounds.iter()).collect::<Vec<_>>();

    assert!(collisions.len() == 3);
}

#[test]
fn sweep_self_quad_overlap() {
    let mut bounds = vec![
        Bounds::from_min_max(Coord2(100.0, 200.0), Coord2(200.0, 300.0)),
        Bounds::from_min_max(Coord2(150.0, 250.0), Coord2(250.0, 350.0)),
        Bounds::from_min_max(Coord2(190.0, 290.0), Coord2(290.0, 390.0)),
        Bounds::from_min_max(Coord2(0.0, 0.0), Coord2(1000.0, 1000.0)),
    ];
    bounds.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions = sweep_self(bounds.iter()).collect::<Vec<_>>();

    assert!(collisions.len() == 6);
}

#[test]
fn sweep_against_single_overlap() {
    let mut bounds1 = vec![
        Bounds::from_min_max(Coord2(100.0, 200.0), Coord2(200.0, 300.0))
    ];
    let mut bounds2 = vec![        
        Bounds::from_min_max(Coord2(150.0, 250.0), Coord2(250.0, 350.0)),
    ];
    bounds1.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));
    bounds2.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions = sweep_against(bounds1.iter(), bounds2.iter()).collect::<Vec<_>>();

    assert!(collisions.len() == 1);
}

#[test]
fn sweep_against_double_overlap_1() {
    let mut bounds1 = vec![
        Bounds::from_min_max(Coord2(100.0, 200.0), Coord2(200.0, 300.0)),
        Bounds::from_min_max(Coord2(220.0, 330.0), Coord2(350.0, 450.0)),
    ];
    let mut bounds2 = vec![        
        Bounds::from_min_max(Coord2(150.0, 250.0), Coord2(250.0, 350.0)),
    ];
    bounds1.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));
    bounds2.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions = sweep_against(bounds1.iter(), bounds2.iter()).collect::<Vec<_>>();

    assert!(collisions.len() == 2);
}

#[test]
fn sweep_against_double_overlap_2() {
    let mut bounds1 = vec![
        Bounds::from_min_max(Coord2(150.0, 250.0), Coord2(250.0, 350.0)),
    ];
    let mut bounds2 = vec![        
        Bounds::from_min_max(Coord2(100.0, 200.0), Coord2(200.0, 300.0)),
        Bounds::from_min_max(Coord2(220.0, 330.0), Coord2(350.0, 450.0)),
    ];
    bounds1.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));
    bounds2.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions = sweep_against(bounds1.iter(), bounds2.iter()).collect::<Vec<_>>();

    assert!(collisions.len() == 2);
}

#[test]
fn sweep_against_quad_overlap() {
    let mut bounds1 = vec![
        Bounds::from_min_max(Coord2(100.0, 200.0), Coord2(200.0, 300.0)),
        Bounds::from_min_max(Coord2(150.0, 250.0), Coord2(250.0, 350.0)),
    ];
    let mut bounds2 = vec![
        Bounds::from_min_max(Coord2(190.0, 290.0), Coord2(290.0, 390.0)),
        Bounds::from_min_max(Coord2(0.0, 0.0), Coord2(1000.0, 1000.0)),
    ];
    bounds1.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));
    bounds2.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions = sweep_against(bounds1.iter(), bounds2.iter()).collect::<Vec<_>>();

    assert!(collisions.len() == 4);
}
