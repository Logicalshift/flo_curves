Change: cache which brush daubs are on which line instead of calculating them

default_algorithm       time:   [1.2393 µs 1.2412 µs 1.2433 µs]
                        change: [+1.7542% +2.0701% +2.3886%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

Benchmarking newton_raphson: Collecting 100 samples in estimated 5.0004 s (9.7M
newton_raphson          time:   [510.28 ns 510.87 ns 511.48 ns]
                        change: [+1.1581% +1.3730% +1.5831%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

     Running benches/rasterize.rs (target/release/deps/rasterize-8361523a02ad8fc1)
Benchmarking scan_convert_circle: Collecting 100 samples in estimated 5.2551 s (
scan_convert_circle     time:   [64.833 µs 64.915 µs 65.002 µs]
                        change: [+1.0733% +1.2772% +1.4775%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Benchmarking scan_convert_curve: Collecting 100 samples in estimated 5.1183 s (1
scan_convert_curve      time:   [30.635 µs 30.673 µs 30.718 µs]
                        change: [+1.4234% +1.6451% +1.8566%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

     Running benches/sweep.rs (target/release/deps/sweep-8775f2c4291112c7)
Benchmarking detect_collisions 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.7s, enable flat sampling, or reduce sample count to 60.
Benchmarking detect_collisions 1000: Collecting 100 samples in estimated 6.7394
detect_collisions 1000  time:   [1.3295 ms 1.3314 ms 1.3334 ms]
                        change: [-0.7711% -0.4163% -0.1111%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

Benchmarking merge_paths 1000: Collecting 100 samples in estimated 9.0366 s (10k
merge_paths 1000        time:   [890.79 µs 892.45 µs 894.56 µs]
                        change: [+0.2872% +0.6056% +0.9267%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

Benchmarking sweep 10: Collecting 100 samples in estimated 5.0002 s (6.1M iterat
sweep 10                time:   [824.19 ns 825.27 ns 826.37 ns]
                        change: [+2.6086% +2.9670% +3.3030%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

Benchmarking sweep_slow 10: Collecting 100 samples in estimated 5.0012 s (7.2M i
sweep_slow 10           time:   [701.25 ns 702.58 ns 703.90 ns]
                        change: [+1.1628% +1.4465% +1.7386%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking sweep 100: Collecting 100 samples in estimated 5.0254 s (505k itera
sweep 100               time:   [9.9233 µs 9.9575 µs 9.9913 µs]
                        change: [+2.8291% +3.1892% +3.5661%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking sweep_slow 100: Collecting 100 samples in estimated 5.0134 s (641k
sweep_slow 100          time:   [7.7923 µs 7.8137 µs 7.8335 µs]
                        change: [-1.4687% -1.0277% -0.6218%] (p = 0.00 < 0.05)
                        Change within noise threshold.

Benchmarking sweep 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.1s, enable flat sampling, or reduce sample count to 60.
Benchmarking sweep 1000: Collecting 100 samples in estimated 6.1161 s (5050 iter
sweep 1000              time:   [1.2102 ms 1.2141 ms 1.2180 ms]
                        change: [-2.3896% -1.6880% -0.9847%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

Benchmarking sweep_slow 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.8s, enable flat sampling, or reduce sample count to 60.
Benchmarking sweep_slow 1000: Collecting 100 samples in estimated 5.7590 s (5050
sweep_slow 1000         time:   [1.1262 ms 1.1332 ms 1.1405 ms]
                        change: [-3.7934% -3.0590% -2.2700%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

     Running benches/vectorize.rs (target/release/deps/vectorize-d7d1aa45de774f03)
Benchmarking offset_curves: Collecting 100 samples in estimated 5.3601 s (56k it
offset_curves           time:   [95.961 µs 96.083 µs 96.203 µs]
                        change: [-1.3993% -1.1063% -0.8185%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe

Benchmarking find_edges 100: Collecting 100 samples in estimated 5.0345 s (187k
find_edges 100          time:   [26.858 µs 26.891 µs 26.927 µs]
                        change: [+0.0915% +0.3293% +0.5696%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Benchmarking find_edges 1000: Collecting 100 samples in estimated 5.0770 s (2000
find_edges 1000         time:   [2.5484 ms 2.5527 ms 2.5573 ms]
                        change: [-0.1768% +0.1198% +0.4178%] (p = 0.44 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking find_edges_not_sampled 100: Collecting 100 samples in estimated 5.0
find_edges_not_sampled 100
                        time:   [4.5822 µs 4.5909 µs 4.5998 µs]
                        change: [+0.0538% +0.2987% +0.5380%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking find_edges_not_sampled 1000: Collecting 100 samples in estimated 5.
find_edges_not_sampled 1000
                        time:   [46.085 µs 46.835 µs 47.546 µs]
                        change: [+1.8599% +3.5909% +5.2510%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 30 outliers among 100 measurements (30.00%)
  12 (12.00%) low severe
  6 (6.00%) low mild
  12 (12.00%) high mild

Benchmarking circle_from_contours 100: Collecting 100 samples in estimated 5.377
circle_from_contours 100
                        time:   [151.90 µs 152.13 µs 152.38 µs]
                        change: [-0.6506% -0.3960% -0.1280%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking circle_from_contours 1000: Collecting 100 samples in estimated 5.39
circle_from_contours 1000
                        time:   [4.1525 ms 4.1596 ms 4.1671 ms]
                        change: [-0.0295% +0.2709% +0.5491%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Benchmarking circle_intercepts_scan_sampled 100: Collecting 100 samples in estim
circle_intercepts_scan_sampled 100
                        time:   [7.4700 µs 7.4876 µs 7.5052 µs]
                        change: [-2.2968% -2.0641% -1.8091%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking circle_intercepts_scan_sampled 1000: Collecting 100 samples in esti
circle_intercepts_scan_sampled 1000
                        time:   [653.83 µs 655.13 µs 656.40 µs]
                        change: [+0.2801% +0.5147% +0.7393%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

Benchmarking circle_intercepts_scan 100: Collecting 100 samples in estimated 5.0
circle_intercepts_scan 100
                        time:   [483.60 ns 491.95 ns 500.15 ns]
                        change: [-5.5784% -3.7464% -1.7608%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking circle_intercepts_scan 1000: Collecting 100 samples in estimated 5.
circle_intercepts_scan 1000
                        time:   [4.6707 µs 5.4632 µs 6.2704 µs]
                        change: [-7.8564% +7.9604% +27.332%] (p = 0.36 > 0.05)
                        No change in performance detected.

Benchmarking circle_start_iteration: Collecting 100 samples in estimated 5.0003
circle_start_iteration  time:   [105.40 ns 105.61 ns 105.84 ns]
                        change: [-1.6862% -1.1393% -0.6126%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

Benchmarking circle_from_contours_not_sampled 100: Collecting 100 samples in est
circle_from_contours_not_sampled 100
                        time:   [126.61 µs 126.80 µs 126.97 µs]
                        change: [-2.2686% -1.8773% -1.5016%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking circle_from_contours_not_sampled 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.8s, enable flat sampling, or reduce sample count to 60.
Benchmarking circle_from_contours_not_sampled 1000: Collecting 100 samples in es
circle_from_contours_not_sampled 1000
                        time:   [1.3352 ms 1.3366 ms 1.3381 ms]
                        change: [-2.3097% -1.9817% -1.6269%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

Benchmarking create_brush_stroke_daubs: Collecting 100 samples in estimated 5.02
create_brush_stroke_daubs
                        time:   [36.978 µs 37.986 µs 38.927 µs]
                        change: [+27.492% +31.757% +35.891%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking create_brush_distance_field: Collecting 100 samples in estimated 8.
create_brush_distance_field
                        time:   [840.15 µs 841.16 µs 842.29 µs]
                        change: [+1255.9% +1262.4% +1268.8%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

Benchmarking start_brush_iteration: Collecting 100 samples in estimated 5.0020 s
start_brush_iteration   time:   [1.6668 µs 1.6681 µs 1.6699 µs]
                        change: [-65.811% -65.743% -65.673%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  9 (9.00%) high severe

Benchmarking brush_intercepts_scan: Collecting 100 samples in estimated 5.1670 s
brush_intercepts_scan   time:   [2.3369 ms 2.3424 ms 2.3476 ms]
                        change: [-33.669% -33.511% -33.356%] (p = 0.00 < 0.05)
                        Performance has improved.

Benchmarking read_brush_stroke_edges: Collecting 100 samples in estimated 5.0038
read_brush_stroke_edges time:   [2.4070 ms 2.4084 ms 2.4097 ms]
                        change: [-33.217% -33.122% -33.027%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 24 outliers among 100 measurements (24.00%)
  4 (4.00%) low severe
  16 (16.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking read_edge_distances: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.0s, enable flat sampling, or reduce sample count to 50.
Benchmarking read_edge_distances: Collecting 100 samples in estimated 8.0130 s (
read_edge_distances     time:   [1.5849 ms 1.5856 ms 1.5866 ms]
                        change: [-49.124% -49.019% -48.909%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) high mild
  14 (14.00%) high severe

Benchmarking trace_distance_field: Collecting 100 samples in estimated 5.5695 s
trace_distance_field    time:   [6.1612 ms 6.1656 ms 6.1706 ms]
                        change: [-38.404% -38.330% -38.260%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe

Benchmarking single_daub: Collecting 100 samples in estimated 5.7351 s (10k iter
single_daub             time:   [561.34 µs 562.67 µs 564.33 µs]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe

Benchmarking full_distance_field: Collecting 100 samples in estimated 5.6433 s (
full_distance_field     time:   [6.9908 ms 7.0071 ms 7.0262 ms]
Found 18 outliers among 100 measurements (18.00%)
  8 (8.00%) high mild
  10 (10.00%) high severe
