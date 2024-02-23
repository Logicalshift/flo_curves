use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;
use flo_draw::*;
use flo_draw::canvas::*;

use std::f32;

fn weights_for_gaussian_blur(sigma: f32, step: f32, count: usize) -> Vec<f32> {
    // Short-circuit the case where count is 0
    if count == 0 { return vec![]; }

    let sigma_squared   = sigma * sigma;

    // Compute the weight at each position
    let uncorrected     = (0..count).into_iter()
        .map(|x| {
            let x = x as f32;
            let x = x * step;
            (1.0/((2.0*f32::consts::PI*sigma_squared).sqrt())) * (f32::consts::E.powf(-(x*x)/(2.0*sigma_squared)))
        })
        .collect::<Vec<_>>();

    // Correct the blur so that the weights all add up to 1
    let sum             = uncorrected[0] + uncorrected.iter().skip(1).fold(0.0, |x, y| x+*y)*2.0;
    let corrected       = uncorrected.into_iter().map(|weight| weight/sum).collect();

    corrected
}

fn filter_gaussian(coords: Vec<Coord2>, radius: f32) -> Vec<Coord2> {
    let sigma   = 0.25;
    let step    = 1.0 / radius;
    let count   = radius.ceil() as usize;
    let weights = weights_for_gaussian_blur(sigma, step, count);
    let weights = weights.iter()
        .skip(1).rev().copied()
        .chain(weights.iter().copied())
        .collect::<Vec<_>>();

    let mut filtered = vec![];
    let count = count as i32;

    for idx in 0..coords.len() {
        let mut pos = Coord2(0.0, 0.0);

        let idx = idx as i32;
        for weight_idx in 0..weights.len() {
            let weight      = weights[weight_idx] as f64;
            let weight_idx  = weight_idx as i32;
            let x           = (idx - (count-1)) + weight_idx;
            let coord       = if x < 0 { coords[0] } else if x >= coords.len() as i32 { coords[coords.len()-1] } else { coords[x as usize] };

            pos = pos + (coord * weight);
        }

        filtered.push(pos);
    }

    filtered
}

fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("Curve fitting demonstration");

        let points = vec![
            Coord2(72.0, 216.0),
            Coord2(73.0, 217.0),
            Coord2(74.0, 218.0),
            Coord2(75.0, 219.0),
            Coord2(74.0, 220.0),
            Coord2(75.0, 221.0),
            Coord2(75.0, 222.0),
            Coord2(75.0, 223.0),
            Coord2(76.0, 224.0),
            Coord2(77.0, 225.0),
            Coord2(77.0, 226.0),
            Coord2(77.0, 227.0),
            Coord2(78.0, 227.0),
            Coord2(79.0, 227.0),
            Coord2(80.0, 228.0),
            Coord2(79.0, 229.0),
            Coord2(80.0, 230.0),
            Coord2(81.0, 231.0),
            Coord2(82.0, 232.0),
            Coord2(82.0, 233.0),
            Coord2(82.0, 234.0),
            Coord2(83.0, 235.0),
            Coord2(83.0, 236.0),
            Coord2(83.0, 237.0),
            Coord2(84.0, 238.0),
            Coord2(85.0, 239.0),
            Coord2(84.0, 240.0),
            Coord2(83.0, 241.0),
            Coord2(82.0, 241.0),
            Coord2(81.0, 241.0),
            Coord2(80.0, 242.0),
            Coord2(80.0, 243.0),
            Coord2(80.0, 244.0),
            Coord2(81.0, 245.0),
            Coord2(80.0, 246.0),
            Coord2(81.0, 247.0),
            Coord2(82.0, 248.0),
            Coord2(83.0, 249.0),
            Coord2(84.0, 250.0),
            Coord2(84.0, 251.0),
            Coord2(84.0, 252.0),
            Coord2(83.0, 251.0),
            Coord2(82.0, 251.0),
            Coord2(81.0, 251.0),
            Coord2(80.0, 250.0),
            Coord2(81.0, 249.0),
            Coord2(80.0, 248.0),
            Coord2(79.0, 247.0),
            Coord2(78.0, 246.0),
            Coord2(78.0, 245.0),
            Coord2(78.0, 244.0),
            Coord2(77.0, 243.0),
            Coord2(76.0, 242.0),
            Coord2(76.0, 241.0),
            Coord2(76.0, 240.0),
            Coord2(75.0, 239.0),
            Coord2(75.0, 238.0),
            Coord2(75.0, 237.0),
            Coord2(74.0, 237.0),
            Coord2(73.0, 237.0),
            Coord2(72.0, 236.0),
            Coord2(73.0, 235.0),
            Coord2(72.0, 234.0),
            Coord2(71.0, 233.0),
            Coord2(71.0, 232.0),
            Coord2(71.0, 231.0),
            Coord2(70.0, 230.0),
            Coord2(69.0, 229.0),
            Coord2(68.0, 228.0),
            Coord2(67.0, 227.0),
            Coord2(66.0, 226.0),
            Coord2(65.0, 225.0),
            Coord2(65.0, 224.0),
            Coord2(65.0, 223.0),
            Coord2(64.0, 223.0),
            Coord2(63.0, 223.0),
            Coord2(64.0, 222.0),
            Coord2(65.0, 222.0),
            Coord2(66.0, 222.0),
            Coord2(65.0, 221.0),
            Coord2(66.0, 220.0),
            Coord2(66.0, 219.0),
            Coord2(66.0, 218.0),
            Coord2(65.0, 217.0),
            Coord2(65.0, 216.0),
            Coord2(65.0, 215.0),
            Coord2(64.0, 214.0),
            Coord2(64.0, 213.0),
            Coord2(64.0, 212.0),
            Coord2(64.0, 211.0),
            Coord2(64.0, 210.0),
            Coord2(63.0, 209.0),
            Coord2(62.0, 208.0),
            Coord2(62.0, 207.0),
            Coord2(62.0, 206.0),
            Coord2(61.0, 205.0),
            Coord2(62.0, 204.0),
            Coord2(61.0, 204.0),
            Coord2(60.0, 204.0),
            Coord2(60.0, 203.0),
            Coord2(60.0, 202.0),
            Coord2(59.0, 201.0),
        ];
        let points = filter_gaussian(points, 5.0);
        let spline = bezier::Curve::fit_from_points(&points, 0.5).unwrap();

        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
            gc.canvas_height(100.0);
            gc.center_region(55.0, 200.0, 80.0, 250.0);

            for Coord2(x, y) in points.iter() {
                gc.new_path();
                gc.fill_color(Color::Rgba(0.2, 0.5, 1.0, 1.0));
                gc.circle(*x as _, *y as _, 0.3);
                gc.fill();
            }

            gc.new_path();
            let sp = spline[0].start_point();
            gc.move_to(sp.x() as _, sp.y() as _);

            for curve in spline {
                let (sp, (cp1, cp2), ep) = curve.all_points();

                gc.bezier_curve_to(ep.x() as _, ep.y() as _, cp1.x() as _, cp1.y() as _, cp2.x() as _, cp2.y() as _);
            }

            gc.fill_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.line_width_pixels(2.0);
            gc.stroke();
        });
    });
}
