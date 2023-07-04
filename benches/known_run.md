     Running benches/sweep.rs (target/release/deps/sweep-8775f2c4291112c7)
Benchmarking detect_collisions 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.8s, enable flat sampling, or reduce sample count to 60.
Benchmarking detect_collisions 1000: Collecting 100 samples in estimated 6.7948
detect_collisions 1000  time:   [1.3412 ms 1.3469 ms 1.3526 ms]

Benchmarking merge_paths 1000: Collecting 100 samples in estimated 9.0488 s (10k
merge_paths 1000        time:   [893.31 µs 897.26 µs 901.58 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking sweep 10: Collecting 100 samples in estimated 5.0031 s (6.0M iterat
sweep 10                time:   [826.53 ns 829.89 ns 833.63 ns]

Benchmarking sweep_slow 10: Collecting 100 samples in estimated 5.0013 s (7.1M i
sweep_slow 10           time:   [704.18 ns 706.92 ns 709.71 ns]

Benchmarking sweep 100: Collecting 100 samples in estimated 5.0028 s (505k itera
sweep 100               time:   [9.9013 µs 9.9516 µs 10.004 µs]

Benchmarking sweep_slow 100: Collecting 100 samples in estimated 5.0131 s (641k
sweep_slow 100          time:   [7.7440 µs 7.7729 µs 7.8048 µs]

Benchmarking sweep 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.2s, enable flat sampling, or reduce sample count to 60.
Benchmarking sweep 1000: Collecting 100 samples in estimated 6.1886 s (5050 iter
sweep 1000              time:   [1.2219 ms 1.2315 ms 1.2409 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking sweep_slow 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.7s, enable flat sampling, or reduce sample count to 60.
Benchmarking sweep_slow 1000: Collecting 100 samples in estimated 5.7097 s (5050
sweep_slow 1000         time:   [1.1343 ms 1.1419 ms 1.1507 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

     Running benches/vectorize.rs (target/release/deps/vectorize-d7d1aa45de774f03)
Benchmarking offset_curves: Collecting 100 samples in estimated 5.3486 s (56k it
offset_curves           time:   [95.636 µs 95.903 µs 96.209 µs]

Benchmarking find_edges 100: Collecting 100 samples in estimated 5.0405 s (404k
find_edges 100          time:   [12.375 µs 12.419 µs 12.466 µs]

Benchmarking find_edges 1000: Collecting 100 samples in estimated 7.1413 s (10k
find_edges 1000         time:   [702.03 µs 704.83 µs 707.99 µs]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

Benchmarking find_edges_not_sampled 100: Collecting 100 samples in estimated 5.0
find_edges_not_sampled 100
                        time:   [4.7167 µs 4.7381 µs 4.7596 µs]

Benchmarking find_edges_not_sampled 1000: Collecting 100 samples in estimated 5.
find_edges_not_sampled 1000
                        time:   [48.772 µs 49.146 µs 49.488 µs]

Benchmarking circle_from_contours 100: Collecting 100 samples in estimated 5.556
circle_from_contours 100
                        time:   [137.22 µs 137.79 µs 138.38 µs]

Benchmarking circle_from_contours 1000: Collecting 100 samples in estimated 5.11
circle_from_contours 1000
                        time:   [2.0570 ms 2.0638 ms 2.0708 ms]

Benchmarking circle_intercepts_scan_sampled 100: Collecting 100 samples in estim
circle_intercepts_scan_sampled 100
                        time:   [7.5329 µs 7.5596 µs 7.5897 µs]

Benchmarking circle_intercepts_scan_sampled 1000: Collecting 100 samples in esti
circle_intercepts_scan_sampled 1000
                        time:   [659.95 µs 662.56 µs 665.47 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Benchmarking circle_intercepts_scan 100: Collecting 100 samples in estimated 5.0
circle_intercepts_scan 100
                        time:   [469.68 ns 471.58 ns 473.70 ns]

Benchmarking circle_intercepts_scan 1000: Collecting 100 samples in estimated 5.
circle_intercepts_scan 1000
                        time:   [10.239 µs 10.527 µs 10.746 µs]
Found 24 outliers among 100 measurements (24.00%)
  13 (13.00%) low severe
  7 (7.00%) low mild
  4 (4.00%) high mild

Benchmarking circle_start_iteration: Collecting 100 samples in estimated 5.0001
circle_start_iteration  time:   [105.20 ns 105.59 ns 105.99 ns]
Found 11 outliers among 100 measurements (11.00%)
  11 (11.00%) high mild

Benchmarking circle_from_contours_not_sampled 100: Collecting 100 samples in est
circle_from_contours_not_sampled 100
                        time:   [122.35 µs 122.87 µs 123.39 µs]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

Benchmarking circle_from_contours_not_sampled 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.0s, enable flat sampling, or reduce sample count to 50.
Benchmarking circle_from_contours_not_sampled 1000: Collecting 100 samples in es
circle_from_contours_not_sampled 1000
                        time:   [1.3735 ms 1.3791 ms 1.3851 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking circle_path_intercepts_scan 1000: Collecting 100 samples in estimat
circle_path_intercepts_scan 1000
                        time:   [107.42 µs 108.73 µs 110.08 µs]
Found 12 outliers among 100 measurements (12.00%)
  11 (11.00%) low severe
  1 (1.00%) high mild

Benchmarking circle_path_trace 1000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.4s, enable flat sampling, or reduce sample count to 50.
Benchmarking circle_path_trace 1000: Collecting 100 samples in estimated 7.4036
circle_path_trace 1000  time:   [1.4601 ms 1.4670 ms 1.4744 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking create_brush_stroke_daubs: Collecting 100 samples in estimated 5.03
create_brush_stroke_daubs
                        time:   [44.784 µs 45.553 µs 46.447 µs]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) low mild
  1 (1.00%) high mild

Benchmarking create_brush_distance_field: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.2s, enable flat sampling, or reduce sample count to 60.
Benchmarking create_brush_distance_field: Collecting 100 samples in estimated 6.
create_brush_distance_field
                        time:   [1.2194 ms 1.2237 ms 1.2284 ms]
Found 17 outliers among 100 measurements (17.00%)
  15 (15.00%) high mild
  2 (2.00%) high severe

Benchmarking start_brush_iteration: Collecting 100 samples in estimated 5.0080 s
start_brush_iteration   time:   [1.6070 µs 1.6139 µs 1.6210 µs]

Benchmarking brush_intercepts_scan: Collecting 100 samples in estimated 5.1562 s
brush_intercepts_scan   time:   [2.3236 ms 2.3359 ms 2.3492 ms]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

Benchmarking read_brush_stroke_edges: Collecting 100 samples in estimated 5.0415
read_brush_stroke_edges time:   [2.4024 ms 2.4090 ms 2.4159 ms]

Benchmarking read_edge_distances: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.8s, enable flat sampling, or reduce sample count to 60.
Benchmarking read_edge_distances: Collecting 100 samples in estimated 6.8409 s (
read_edge_distances     time:   [1.3550 ms 1.3606 ms 1.3664 ms]

Benchmarking trace_distance_field: Collecting 100 samples in estimated 5.1690 s
trace_distance_field    time:   [5.7182 ms 5.7425 ms 5.7687 ms]
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe

Benchmarking single_daub: Collecting 100 samples in estimated 5.5959 s (10k iter
single_daub             time:   [552.47 µs 554.90 µs 557.35 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild

Benchmarking single_small_daub: Collecting 100 samples in estimated 5.0747 s (27
single_small_daub       time:   [18.906 µs 18.975 µs 19.048 µs]

Benchmarking ten_daubs: Collecting 100 samples in estimated 6.6432 s (10k iterat
ten_daubs               time:   [657.19 µs 660.83 µs 665.02 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

Benchmarking hundred_daubs: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.2s, enable flat sampling, or reduce sample count to 50.
Benchmarking hundred_daubs: Collecting 100 samples in estimated 8.2436 s (5050 i
hundred_daubs           time:   [1.6226 ms 1.6295 ms 1.6372 ms]

Benchmarking hundred_small_daubs: Collecting 100 samples in estimated 5.1889 s (
hundred_small_daubs     time:   [128.31 µs 128.86 µs 129.47 µs]

Benchmarking hundred_daubs_horiz: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.4s, enable flat sampling, or reduce sample count to 50.
Benchmarking hundred_daubs_horiz: Collecting 100 samples in estimated 8.3587 s (
hundred_daubs_horiz     time:   [1.6430 ms 1.6482 ms 1.6540 ms]

Benchmarking hundred_small_daubs_horiz: Collecting 100 samples in estimated 5.59
hundred_small_daubs_horiz
                        time:   [123.49 µs 123.88 µs 124.30 µs]

Benchmarking full_distance_field: Collecting 100 samples in estimated 5.5674 s (
full_distance_field     time:   [6.9328 ms 6.9592 ms 6.9885 ms]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking full_distance_field_small_brush: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.8s, enable flat sampling, or reduce sample count to 50.
Benchmarking full_distance_field_small_brush: Collecting 100 samples in estimate
full_distance_field_small_brush
                        time:   [1.9383 ms 1.9480 ms 1.9585 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
