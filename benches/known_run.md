Change: make find_intercepts a floating-point operation

 Benchmarking default_algorithm: Collecting 100 samples in estimated 5.0003 s (4.
default_algorithm       time:   [1.2119 µs 1.2140 µs 1.2163 µs]
                        change: [-1.9130% -1.6278% -1.3193%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe

Benchmarking newton_raphson: Collecting 100 samples in estimated 5.0022 s (9.9M
newton_raphson          time:   [504.44 ns 504.84 ns 505.35 ns]
                        change: [-1.6526% -1.5014% -1.3270%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

     Running benches/rasterize.rs (target/release/deps/rasterize-8361523a02ad8fc1)
Benchmarking scan_convert_circle: Collecting 100 samples in estimated 5.1779 s (
scan_convert_circle     time:   [64.000 µs 64.059 µs 64.135 µs]
                        change: [-1.6788% -1.5257% -1.3782%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

Benchmarking scan_convert_curve: Collecting 100 samples in estimated 5.0228 s (1
scan_convert_curve      time:   [30.109 µs 30.122 µs 30.138 µs]
                        change: [-1.6296% -1.4447% -1.2230%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

     Running benches/sweep.rs (target/release/deps/sweep-8775f2c4291112c7)
Benchmarking detect_collisions 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.7s, enable flat sampling, or reduce sample count to 60.
Benchmarking detect_collisions 1000: Collecting 100 samples in estimated 6.7467
detect_collisions 1000  time:   [1.3366 ms 1.3388 ms 1.3417 ms]
                        change: [-2.1548% -1.7105% -1.3047%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 28 outliers among 100 measurements (28.00%)
  16 (16.00%) low mild
  12 (12.00%) high severe

Benchmarking merge_paths 1000: Collecting 100 samples in estimated 8.8731 s (10k
merge_paths 1000        time:   [878.66 µs 879.56 µs 880.68 µs]
                        change: [-1.1310% -0.7759% -0.4513%] (p = 0.00 < 0.05)
                        Change within noise threshold.

Benchmarking sweep 10: Collecting 100 samples in estimated 5.0040 s (6.1M iterat
sweep 10                time:   [803.45 ns 807.05 ns 810.64 ns]
                        change: [-3.7642% -3.4209% -3.0847%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking sweep_slow 10: Collecting 100 samples in estimated 5.0021 s (7.3M i
sweep_slow 10           time:   [690.38 ns 691.27 ns 692.30 ns]
                        change: [-1.2976% -1.0876% -0.8570%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

Benchmarking sweep 100: Collecting 100 samples in estimated 5.0026 s (515k itera
sweep 100               time:   [9.6793 µs 9.6963 µs 9.7142 µs]
                        change: [-3.0040% -2.6912% -2.3755%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking sweep_slow 100: Collecting 100 samples in estimated 5.0308 s (646k
sweep_slow 100          time:   [7.8065 µs 7.8217 µs 7.8391 µs]
                        change: [+0.7751% +1.1566% +1.5685%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Benchmarking sweep 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.2s, enable flat sampling, or reduce sample count to 60.
Benchmarking sweep 1000: Collecting 100 samples in estimated 6.2385 s (5050 iter
sweep 1000              time:   [1.2177 ms 1.2236 ms 1.2299 ms]
                        change: [-2.8628% -1.9668% -0.9895%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Benchmarking sweep_slow 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.8s, enable flat sampling, or reduce sample count to 60.
Benchmarking sweep_slow 1000: Collecting 100 samples in estimated 5.7984 s (5050
sweep_slow 1000         time:   [1.1652 ms 1.1701 ms 1.1752 ms]
                        change: [+3.0383% +3.8896% +4.7022%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

     Running benches/vectorize.rs (target/release/deps/vectorize-d7d1aa45de774f03)
Benchmarking offset_curves: Collecting 100 samples in estimated 5.4061 s (56k it
offset_curves           time:   [97.078 µs 97.281 µs 97.503 µs]
                        change: [+0.2694% +0.5668% +0.8721%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking find_edges 100: Collecting 100 samples in estimated 5.0067 s (187k
find_edges 100          time:   [26.864 µs 26.924 µs 26.995 µs]
                        change: [-0.8466% -0.6507% -0.4387%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking find_edges 1000: Collecting 100 samples in estimated 5.0954 s (2000
find_edges 1000         time:   [2.5437 ms 2.5497 ms 2.5560 ms]
                        change: [-0.5273% -0.2185% +0.1181%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe

Benchmarking find_edges_not_sampled 100: Collecting 100 samples in estimated 5.0
find_edges_not_sampled 100
                        time:   [4.5698 µs 4.5790 µs 4.5886 µs]
                        change: [+0.1137% +0.3749% +0.6299%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

Benchmarking find_edges_not_sampled 1000: Collecting 100 samples in estimated 5.
find_edges_not_sampled 1000
                        time:   [44.970 µs 45.484 µs 45.968 µs]
                        change: [-5.4829% -3.5657% -1.5480%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking circle_from_contours 100: Collecting 100 samples in estimated 5.401
circle_from_contours 100
                        time:   [151.87 µs 152.17 µs 152.51 µs]
                        change: [-1.2797% -0.8626% -0.4608%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

Benchmarking circle_from_contours 1000: Collecting 100 samples in estimated 5.04
circle_from_contours 1000
                        time:   [4.1396 ms 4.1484 ms 4.1582 ms]
                        change: [-0.6744% -0.3719% -0.0659%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking circle_intercepts_scan_sampled 100: Collecting 100 samples in estim
circle_intercepts_scan_sampled 100
                        time:   [7.6038 µs 7.6158 µs 7.6286 µs]
                        change: [+3.5629% +3.7794% +4.0066%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking circle_intercepts_scan_sampled 1000: Collecting 100 samples in esti
circle_intercepts_scan_sampled 1000
                        time:   [651.60 µs 652.36 µs 653.16 µs]
                        change: [-0.9121% -0.6804% -0.4684%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe

Benchmarking circle_intercepts_scan 100: Collecting 100 samples in estimated 5.0
circle_intercepts_scan 100
                        time:   [499.33 ns 511.01 ns 522.40 ns]
                        change: [+8.5607% +10.743% +13.016%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking circle_intercepts_scan 1000: Collecting 100 samples in estimated 5.
circle_intercepts_scan 1000
                        time:   [4.1123 µs 4.6290 µs 5.2114 µs]
                        change: [-47.405% -41.681% -35.120%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking circle_start_iteration: Collecting 100 samples in estimated 5.0002
circle_start_iteration  time:   [105.81 ns 106.35 ns 106.94 ns]
                        change: [+1.2038% +1.7031% +2.1901%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

Benchmarking circle_from_contours_not_sampled 100: Collecting 100 samples in est
circle_from_contours_not_sampled 100
                        time:   [129.21 µs 129.71 µs 130.25 µs]
                        change: [-0.6661% -0.1485% +0.3508%] (p = 0.58 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking circle_from_contours_not_sampled 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.9s, enable flat sampling, or reduce sample count to 50.
Benchmarking circle_from_contours_not_sampled 1000: Collecting 100 samples in es
circle_from_contours_not_sampled 1000
                        time:   [1.3582 ms 1.3608 ms 1.3635 ms]
                        change: [-0.2454% +0.0629% +0.3825%] (p = 0.71 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

Benchmarking create_brush_stroke_daubs: Collecting 100 samples in estimated 5.11
create_brush_stroke_daubs
                        time:   [28.629 µs 29.348 µs 29.977 µs]
                        change: [-9.0271% -4.5186% +0.2758%] (p = 0.07 > 0.05)
                        No change in performance detected.

Benchmarking create_brush_distance_field: Collecting 100 samples in estimated 5.
create_brush_distance_field
                        time:   [61.330 µs 61.570 µs 61.816 µs]
                        change: [-92.742% -92.707% -92.670%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

Benchmarking start_brush_iteration: Collecting 100 samples in estimated 5.0123 s
start_brush_iteration   time:   [4.8530 µs 4.8625 µs 4.8720 µs]
                        change: [+0.9881% +1.2764% +1.5553%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking brush_intercepts_scan: Collecting 100 samples in estimated 5.2820 s
brush_intercepts_scan   time:   [3.5197 ms 3.5230 ms 3.5265 ms]
                        change: [+47.918% +48.190% +48.474%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking read_brush_stroke_edges: Collecting 100 samples in estimated 5.0392
read_brush_stroke_edges time:   [3.5965 ms 3.6011 ms 3.6059 ms]
                        change: [-0.7795% -0.5070% -0.2447%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking read_edge_distances: Collecting 100 samples in estimated 5.2805 s (
read_edge_distances     time:   [3.1128 ms 3.1171 ms 3.1217 ms]
                        change: [+0.2032% +0.3765% +0.5419%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking trace_distance_field: Collecting 100 samples in estimated 5.0668 s
trace_distance_field    time:   [9.9898 ms 9.9978 ms 10.007 ms]
                        change: [+59.567% +59.820% +60.064%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
