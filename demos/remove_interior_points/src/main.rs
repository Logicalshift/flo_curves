use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;
use flo_draw::*;
use flo_draw::canvas::*;

use std::f64;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    with_2d_graphics(|| {
        loop {
            canvas.draw(|gc| {

            });

            thread::sleep(Duration::from_nanos(1_000_000_000 / 60));
        }
    });
}
