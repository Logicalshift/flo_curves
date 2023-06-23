Change: make find_intercepts a floating-point operation

     Running benches/nearest_point.rs (target/release/deps/nearest_point-bc75bf487ab70ef7)
Benchmarking default_algorithm: Collecting 100 samples in estimated 5.0062 s (4.
default_algorithm       time:   [1.2369 µs 1.2443 µs 1.2521 µs]
                        change: [-0.8182% -0.4949% -0.1128%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  9 (9.00%) high mild
  5 (5.00%) high severe

Benchmarking newton_raphson: Collecting 100 samples in estimated 5.0012 s (9.7M
newton_raphson          time:   [512.90 ns 514.30 ns 515.94 ns]
                        change: [+0.1661% +0.3814% +0.6096%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe

     Running benches/rasterize.rs (target/release/deps/rasterize-8361523a02ad8fc1)
Benchmarking scan_convert_circle: Collecting 100 samples in estimated 5.2577 s (
scan_convert_circle     time:   [64.972 µs 65.155 µs 65.371 µs]
                        change: [-0.2979% -0.0113% +0.2935%] (p = 0.94 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Benchmarking scan_convert_curve: Collecting 100 samples in estimated 5.1345 s (1
scan_convert_curve      time:   [30.535 µs 30.622 µs 30.734 µs]
                        change: [-0.3586% -0.1394% +0.0737%] (p = 0.23 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
  4 (4.00%) high severe

     Running benches/sweep.rs (target/release/deps/sweep-8775f2c4291112c7)
Benchmarking detect_collisions 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.7s, enable flat sampling, or reduce sample count to 60.
Benchmarking detect_collisions 1000: Collecting 100 samples in estimated 6.7413
detect_collisions 1000  time:   [1.3319 ms 1.3343 ms 1.3371 ms]
                        change: [-0.4234% -0.1497% +0.1505%] (p = 0.29 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high severe

Benchmarking merge_paths 1000: Collecting 100 samples in estimated 8.9950 s (10k
merge_paths 1000        time:   [892.39 µs 896.31 µs 900.39 µs]
                        change: [-0.2473% -0.0067% +0.2262%] (p = 0.96 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

Benchmarking sweep 10: Collecting 100 samples in estimated 5.0014 s (6.1M iterat
sweep 10                time:   [821.41 ns 823.55 ns 825.98 ns]
                        change: [-0.2571% -0.0338% +0.1957%] (p = 0.77 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Benchmarking sweep_slow 10: Collecting 100 samples in estimated 5.0031 s (7.1M i
sweep_slow 10           time:   [695.84 ns 697.30 ns 698.90 ns]
                        change: [-0.6183% -0.4427% -0.2555%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

Benchmarking sweep 100: Collecting 100 samples in estimated 5.0478 s (510k itera
sweep 100               time:   [9.8939 µs 9.9285 µs 9.9636 µs]
                        change: [+0.8983% +1.2121% +1.5706%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking sweep_slow 100: Collecting 100 samples in estimated 5.0382 s (646k
sweep_slow 100          time:   [7.7206 µs 7.7508 µs 7.7827 µs]
                        change: [-0.4099% -0.1170% +0.1951%] (p = 0.42 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe

Benchmarking sweep 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.2s, enable flat sampling, or reduce sample count to 60.
Benchmarking sweep 1000: Collecting 100 samples in estimated 6.2197 s (5050 iter
sweep 1000              time:   [1.2246 ms 1.2307 ms 1.2373 ms]
                        change: [+2.0374% +2.5002% +2.9686%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking sweep_slow 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.7s, enable flat sampling, or reduce sample count to 60.
Benchmarking sweep_slow 1000: Collecting 100 samples in estimated 5.7333 s (5050
sweep_slow 1000         time:   [1.1277 ms 1.1343 ms 1.1410 ms]
                        change: [+0.9686% +1.4831% +2.1164%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

     Running benches/vectorize.rs (target/release/deps/vectorize-d7d1aa45de774f03)
Benchmarking find_edges 100: Collecting 100 samples in estimated 5.0557 s (187k
find_edges 100          time:   [26.838 µs 26.892 µs 26.952 µs]
                        change: [-0.5723% -0.4028% -0.2364%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  2 (2.00%) high severe

Benchmarking find_edges 1000: Collecting 100 samples in estimated 5.0652 s (2000
find_edges 1000         time:   [2.5185 ms 2.5226 ms 2.5269 ms]
                        change: [-1.2336% -1.0347% -0.8329%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Benchmarking find_edges_not_sampled 100: Collecting 100 samples in estimated 5.0
find_edges_not_sampled 100
                        time:   [4.5513 µs 4.5581 µs 4.5671 µs]
                        change: [+77.161% +77.875% +78.679%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

Benchmarking find_edges_not_sampled 1000: Collecting 100 samples in estimated 5.
find_edges_not_sampled 1000
                        time:   [41.575 µs 41.768 µs 42.014 µs]
                        change: [+41.136% +42.779% +44.481%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 15 outliers among 100 measurements (15.00%)
  11 (11.00%) high mild
  4 (4.00%) high severe

Benchmarking circle_from_contours 100: Collecting 100 samples in estimated 5.511
circle_from_contours 100
                        time:   [156.54 µs 157.02 µs 157.54 µs]
                        change: [+2.1417% +2.4846% +2.8309%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking circle_from_contours 1000: Collecting 100 samples in estimated 5.29
circle_from_contours 1000
                        time:   [4.3352 ms 4.3512 ms 4.3673 ms]
                        change: [+4.4620% +4.8574% +5.2239%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking circle_intercepts_scan_sampled 100: Collecting 100 samples in estim
circle_intercepts_scan_sampled 100
                        time:   [7.6795 µs 7.7079 µs 7.7404 µs]
                        change: [+5.2781% +5.6176% +5.9881%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Benchmarking circle_intercepts_scan_sampled 1000: Collecting 100 samples in esti
circle_intercepts_scan_sampled 1000
                        time:   [653.91 µs 656.08 µs 658.54 µs]
                        change: [+0.5514% +0.8952% +1.2329%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

Benchmarking circle_intercepts_scan 100: Collecting 100 samples in estimated 5.0
circle_intercepts_scan 100
                        time:   [391.29 ns 394.71 ns 397.94 ns]
                        change: [-31.984% -31.380% -30.672%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking circle_intercepts_scan 1000: Collecting 100 samples in estimated 5.
circle_intercepts_scan 1000
                        time:   [2.9400 µs 3.1202 µs 3.3283 µs]
                        change: [-77.577% -76.107% -74.607%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Benchmarking circle_start_iteration: Collecting 100 samples in estimated 5.0004
circle_start_iteration  time:   [105.19 ns 105.49 ns 105.85 ns]
                        change: [+933.85% +942.61% +949.45%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking circle_from_contours_not_sampled 100: Collecting 100 samples in est
circle_from_contours_not_sampled 100
                        time:   [133.22 µs 133.46 µs 133.75 µs]
                        change: [-4.2945% -3.9866% -3.6792%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

Benchmarking circle_from_contours_not_sampled 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.1s, enable flat sampling, or reduce sample count to 50.
Benchmarking circle_from_contours_not_sampled 1000: Collecting 100 samples in es
circle_from_contours_not_sampled 1000
                        time:   [1.3997 ms 1.4033 ms 1.4071 ms]
                        change: [-4.2008% -3.0849% -2.2316%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

Benchmarking create_brush_stroke_daubs: Collecting 100 samples in estimated 5.01
create_brush_stroke_daubs
                        time:   [27.070 µs 27.986 µs 28.842 µs]
                        change: [-25.609% -23.645% -21.744%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking create_brush_distance_field: Collecting 100 samples in estimated 5.
create_brush_distance_field
                        time:   [58.401 µs 58.867 µs 59.331 µs]
                        change: [-8.4337% -7.3479% -6.1517%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

Benchmarking start_brush_iteration: Collecting 100 samples in estimated 5.0003 s
start_brush_iteration   time:   [3.9142 µs 3.9214 µs 3.9287 µs]
                        change: [-70.420% -70.369% -70.316%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking brush_intercepts_scan: Collecting 100 samples in estimated 5.0814 s
brush_intercepts_scan   time:   [2.4094 ms 2.4124 ms 2.4158 ms]
                        change: [-36.048% -35.964% -35.866%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe

Benchmarking read_brush_stroke_edges: Collecting 100 samples in estimated 5.1758
read_brush_stroke_edges time:   [2.4483 ms 2.4519 ms 2.4555 ms]
                        change: [-35.449% -35.337% -35.222%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking read_edge_distances: Collecting 100 samples in estimated 5.2130 s (
read_edge_distances     time:   [3.0675 ms 3.0718 ms 3.0767 ms]
                        change: [-1.6265% -1.4416% -1.2463%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  3 (3.00%) high mild
  15 (15.00%) high severe

Benchmarking trace_distance_field: Collecting 100 samples in estimated 5.3708 s
trace_distance_field    time:   [8.9644 ms 8.9758 ms 8.9877 ms]
                        change: [-12.494% -12.377% -12.257%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
