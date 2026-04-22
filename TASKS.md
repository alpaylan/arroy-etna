# arroy — ETNA Tasks

Total tasks: 12

## Task Index

| Task | Variant | Framework | Property | Witness |
|------|---------|-----------|----------|---------|
| 001 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | proptest | `BqEuclidSelfDistanceZero` | `witness_bq_euclid_self_distance_zero_case_small_positive` |
| 002 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | quickcheck | `BqEuclidSelfDistanceZero` | `witness_bq_euclid_self_distance_zero_case_small_positive` |
| 003 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | crabcheck | `BqEuclidSelfDistanceZero` | `witness_bq_euclid_self_distance_zero_case_small_positive` |
| 004 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | hegel | `BqEuclidSelfDistanceZero` | `witness_bq_euclid_self_distance_zero_case_small_positive` |
| 005 | `bq_len_bits_vs_bytes_facc857_1` | proptest | `BqLenMatchesIter` | `witness_bq_len_matches_iter_case_five_floats` |
| 006 | `bq_len_bits_vs_bytes_facc857_1` | quickcheck | `BqLenMatchesIter` | `witness_bq_len_matches_iter_case_five_floats` |
| 007 | `bq_len_bits_vs_bytes_facc857_1` | crabcheck | `BqLenMatchesIter` | `witness_bq_len_matches_iter_case_five_floats` |
| 008 | `bq_len_bits_vs_bytes_facc857_1` | hegel | `BqLenMatchesIter` | `witness_bq_len_matches_iter_case_five_floats` |
| 009 | `cosine_distance_no_clamp_6a09118_1` | proptest | `CosineDistanceInUnit` | `witness_cosine_distance_in_unit_case_identical_ones` |
| 010 | `cosine_distance_no_clamp_6a09118_1` | quickcheck | `CosineDistanceInUnit` | `witness_cosine_distance_in_unit_case_identical_ones` |
| 011 | `cosine_distance_no_clamp_6a09118_1` | crabcheck | `CosineDistanceInUnit` | `witness_cosine_distance_in_unit_case_identical_ones` |
| 012 | `cosine_distance_no_clamp_6a09118_1` | hegel | `CosineDistanceInUnit` | `witness_cosine_distance_in_unit_case_identical_ones` |

## Witness Catalog

- `witness_bq_euclid_self_distance_zero_case_small_positive` — base passes, variant fails
- `witness_bq_len_matches_iter_case_five_floats` — base passes, variant fails
- `witness_cosine_distance_in_unit_case_identical_ones` — base passes, variant fails
