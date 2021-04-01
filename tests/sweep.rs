use flo_curves::geo::*;

use rand::prelude::*;
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

#[test]
fn sweep_self_1000_random() {
    let mut rng     = StdRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
    let mut bounds  = (0..1000).into_iter()
        .map(|_| {
            let x = rng.gen::<f64>() * 900.0;
            let y = rng.gen::<f64>() * 900.0;
            let w = rng.gen::<f64>() * 400.0;
            let h = rng.gen::<f64>() * 400.0;

            Bounds::from_min_max(Coord2(x, y), Coord2(x+w, y+h))
        })
        .collect::<Vec<_>>();
    bounds.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let collisions  = sweep_self(bounds.iter()).collect::<Vec<_>>();

    // Use the slow approach to detecting the collisions to test against
    let mut slow_collisions = vec![];

    for i1 in 0..bounds.len() {
        for i2 in 0..i1 {
            if i1 == i2 { continue; }

            if bounds[i1].overlaps(&bounds[i2]) {
                slow_collisions.push((&bounds[i1], &bounds[i2]));
            }
        }
    }

    assert!(collisions.len() == slow_collisions.len());
}
