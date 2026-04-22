# arroy — Injected Bugs

Total mutations: 3

## Bug Index

| # | Variant | Name | Location | Injection | Fix Commit |
|---|---------|------|----------|-----------|------------|
| 1 | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | `bq_euclid_self_distance_or_not_xor` | `src/distance/binary_quantized_euclidean.rs` | `patch` | `47fd2bdd7f59efa84ddd9ea5dcb1b10c32b4849c` |
| 2 | `bq_len_bits_vs_bytes_facc857_1` | `bq_len_bits_vs_bytes` | `src/unaligned_vector/binary_quantized.rs` | `patch` | `facc8575222d3f5da5b9a94288e44896911e701f` |
| 3 | `cosine_distance_no_clamp_6a09118_1` | `cosine_distance_no_clamp` | `src/distance/cosine.rs` | `patch` | `6a091180d7558d464cdd2ce9daec35e055e1d91c` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | `BqEuclidSelfDistanceZero` | `witness_bq_euclid_self_distance_zero_case_small_positive` |
| `bq_len_bits_vs_bytes_facc857_1` | `BqLenMatchesIter` | `witness_bq_len_matches_iter_case_five_floats` |
| `cosine_distance_no_clamp_6a09118_1` | `CosineDistanceInUnit` | `witness_cosine_distance_in_unit_case_identical_ones` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `BqEuclidSelfDistanceZero` | ✓ | ✓ | ✓ | ✓ |
| `BqLenMatchesIter` | ✓ | ✓ | ✓ | ✓ |
| `CosineDistanceInUnit` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. bq_euclid_self_distance_or_not_xor

- **Variant**: `bq_euclid_self_distance_or_not_xor_47fd2bd_1`
- **Location**: `src/distance/binary_quantized_euclidean.rs`
- **Property**: `BqEuclidSelfDistanceZero`
- **Witness(es)**:
  - `witness_bq_euclid_self_distance_zero_case_small_positive`
- **Source**: fix the normalized distance for the binary quantized euclidean distance
  > `BinaryQuantizedEuclidean::built_distance` computed the popcount over `u | v` instead of `u ^ v`, so `built_distance(p, p)` became `4 * popcount(p)` instead of `0` — violating the self-identity axiom for any non-zero leaf.
- **Fix commit**: `47fd2bdd7f59efa84ddd9ea5dcb1b10c32b4849c` — fix the normalized distance for the binary quantized euclidean distance
- **Invariant violated**: `BinaryQuantizedEuclidean::built_distance(p, p)` must be `0.0` for every leaf (self-identity axiom of a distance metric).
- **How the mutation triggers**: The popcount argument changes from `u ^ v` (XOR, which gives `0` for `u == v`) to `u | v` (OR, which equals `popcount(v)` for `u == v`). Any leaf with at least one positive scalar then reports `built_distance(p, p) = 4 * popcount(v) > 0`.

### 2. bq_len_bits_vs_bytes

- **Variant**: `bq_len_bits_vs_bytes_facc857_1`
- **Location**: `src/unaligned_vector/binary_quantized.rs`
- **Property**: `BqLenMatchesIter`
- **Witness(es)**:
  - `witness_bq_len_matches_iter_case_five_floats`
- **Source**: fix the size of the iterator
  > `UnalignedVector<BinaryQuantized>::len()` returned a word count instead of a bit count, so the `ExactSizeIterator` length was 64× smaller than the number of f32s actually yielded by `iter()` and `to_vec()`.
- **Fix commit**: `facc8575222d3f5da5b9a94288e44896911e701f` — fix the size of the iterator
- **Invariant violated**: `UnalignedVector<BinaryQuantized>::len()` must equal the number of items produced by `iter()` and `to_vec()` (ExactSizeIterator contract).
- **How the mutation triggers**: `len()` returns `vec.vector.len() / QUANTIZED_WORD_BYTES` (a word count) instead of `(vec.vector.len() / QUANTIZED_WORD_BYTES) * QUANTIZED_WORD_BITS` (a bit count). The iterator still yields `words * 64` f32s, so `len()` falls out of sync by a factor of 64.

### 3. cosine_distance_no_clamp

- **Variant**: `cosine_distance_no_clamp_6a09118_1`
- **Location**: `src/distance/cosine.rs`
- **Property**: `CosineDistanceInUnit`
- **Witness(es)**:
  - `witness_cosine_distance_in_unit_case_identical_ones`
- **Source**: Numerical stability improvements for Cosine Distance
  > `Cosine::built_distance` could return a negative value because the pre-fix body neither guarded the divisor against tiny `pnqn` values nor clamped the cosine to `[-1.0, 1.0]` before computing `1 - cos`. The fix strengthens the guard to `f32::EPSILON` and clamps the cosine before subtraction.
- **Fix commit**: `6a091180d7558d464cdd2ce9daec35e055e1d91c` — Numerical stability improvements for Cosine Distance
- **Invariant violated**: `Cosine::built_distance(p, q)` must be finite and within `[0.0, 1.0]` for any pair of sanitized, non-empty vectors.
- **How the mutation triggers**: The division guard is weakened from `pnqn > f32::EPSILON` to `pnqn != 0.0`, and the subsequent `cos.clamp(-1.0, 1.0)` is removed. In f32 `sqrt(7)^2 != 7`, so the cosine of seven-ones against itself can land slightly above 1.0, making the distance `1 - cos` negative.
