# arroy — ETNA Tasks

Total tasks: 12

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task. The `<PropertyKey>` token in the command column uses the PascalCase key recognised by `src/bin/etna.rs`; passing `All` runs every property for the named framework in a single invocation.

## Property keys

| Property | PropertyKey |
|----------|-------------|
| `property_bq_len_matches_iter` | `BqLenMatchesIter` |
| `property_cosine_distance_in_unit` | `CosineDistanceInUnit` |
| `property_bq_euclid_self_distance_zero` | `BqEuclidSelfDistanceZero` |

## Task Index

| Task | Variant | Framework | Property | Witness | Command |
|------|---------|-----------|----------|---------|---------|
| 001 | `bq_len_bits_vs_bytes_facc857_1` | proptest   | `property_bq_len_matches_iter` | `witness_bq_len_matches_iter_case_five_floats` | `cargo run --release --bin etna -- proptest BqLenMatchesIter` |
| 002 | `bq_len_bits_vs_bytes_facc857_1` | quickcheck | `property_bq_len_matches_iter` | `witness_bq_len_matches_iter_case_five_floats` | `cargo run --release --bin etna -- quickcheck BqLenMatchesIter` |
| 003 | `bq_len_bits_vs_bytes_facc857_1` | crabcheck  | `property_bq_len_matches_iter` | `witness_bq_len_matches_iter_case_five_floats` | `cargo run --release --bin etna -- crabcheck BqLenMatchesIter` |
| 004 | `bq_len_bits_vs_bytes_facc857_1` | hegel      | `property_bq_len_matches_iter` | `witness_bq_len_matches_iter_case_five_floats` | `cargo run --release --bin etna -- hegel BqLenMatchesIter` |
| 005 | `cosine_distance_no_clamp_6a09118_1` | proptest   | `property_cosine_distance_in_unit` | `witness_cosine_distance_in_unit_case_identical_ones` | `cargo run --release --bin etna -- proptest CosineDistanceInUnit` |
| 006 | `cosine_distance_no_clamp_6a09118_1` | quickcheck | `property_cosine_distance_in_unit` | `witness_cosine_distance_in_unit_case_identical_ones` | `cargo run --release --bin etna -- quickcheck CosineDistanceInUnit` |
| 007 | `cosine_distance_no_clamp_6a09118_1` | crabcheck  | `property_cosine_distance_in_unit` | `witness_cosine_distance_in_unit_case_identical_ones` | `cargo run --release --bin etna -- crabcheck CosineDistanceInUnit` |
| 008 | `cosine_distance_no_clamp_6a09118_1` | hegel      | `property_cosine_distance_in_unit` | `witness_cosine_distance_in_unit_case_identical_ones` | `cargo run --release --bin etna -- hegel CosineDistanceInUnit` |
| 009 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | proptest   | `property_bq_euclid_self_distance_zero` | `witness_bq_euclid_self_distance_zero_case_small_positive` | `cargo run --release --bin etna -- proptest BqEuclidSelfDistanceZero` |
| 010 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | quickcheck | `property_bq_euclid_self_distance_zero` | `witness_bq_euclid_self_distance_zero_case_small_positive` | `cargo run --release --bin etna -- quickcheck BqEuclidSelfDistanceZero` |
| 011 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | crabcheck  | `property_bq_euclid_self_distance_zero` | `witness_bq_euclid_self_distance_zero_case_small_positive` | `cargo run --release --bin etna -- crabcheck BqEuclidSelfDistanceZero` |
| 012 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | hegel      | `property_bq_euclid_self_distance_zero` | `witness_bq_euclid_self_distance_zero_case_small_positive` | `cargo run --release --bin etna -- hegel BqEuclidSelfDistanceZero` |

## Witness catalog

Each witness is a deterministic concrete test. Base build: passes. Variant-active build: fails.

- `witness_bq_len_matches_iter_case_five_floats` — `property_bq_len_matches_iter(vec![-1.0, 2.0, -3.0, 4.0, 5.0])` → `Pass` (five-element slice quantizes to one 64-bit word; iterator yields 64 values. Buggy `len()` returns `1`, which contradicts `iter().count() == 64`).
- `witness_cosine_distance_in_unit_case_identical_ones` — `property_cosine_distance_in_unit(vec![1.0; 7], vec![1.0; 7])` → `Pass` (f32 rounding in `sqrt(7)^2 != 7` makes the unclamped cosine slightly greater than 1.0, so the buggy distance `1 - cos < 0`).
- `witness_bq_euclid_self_distance_zero_case_small_positive` — `property_bq_euclid_self_distance_zero(vec![1.0, -2.0, 3.0])` → `Pass` (two positive bits → `popcount(v|v) = 2`, so the buggy self-distance is `4 * 2 = 8.0` instead of `0.0`).
