# arroy — Injected Bugs

Total mutations: 3

## Bug Index

| # | Name | Variant | File | Injection | Fix Commit |
|---|------|---------|------|-----------|------------|
| 1 | `bq_len_bits_vs_bytes` | `bq_len_bits_vs_bytes_facc857_1` | `patches/bq_len_bits_vs_bytes_facc857_1.patch` | `patch` | `facc8575222d3f5da5b9a94288e44896911e701f` |
| 2 | `cosine_distance_no_clamp` | `cosine_distance_no_clamp_6a09118_1` | `patches/cosine_distance_no_clamp_6a09118_1.patch` | `patch` | `6a091180d7558d464cdd2ce9daec35e055e1d91c` |
| 3 | `bq_euclid_self_distance_or_not_xor` | `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | `patches/bq_euclid_self_distance_or_not_xor_47fd2bd_1.patch` | `patch` | `47fd2bdd7f59efa84ddd9ea5dcb1b10c32b4849c` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `bq_len_bits_vs_bytes_facc857_1` | `property_bq_len_matches_iter` | `witness_bq_len_matches_iter_case_five_floats` |
| `cosine_distance_no_clamp_6a09118_1` | `property_cosine_distance_in_unit` | `witness_cosine_distance_in_unit_case_identical_ones` |
| `bq_euclid_self_distance_or_not_xor_47fd2bd_1` | `property_bq_euclid_self_distance_zero` | `witness_bq_euclid_self_distance_zero_case_small_positive` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `property_bq_len_matches_iter` | ✓ | ✓ | ✓ | ✓ |
| `property_cosine_distance_in_unit` | ✓ | ✓ | ✓ | ✓ |
| `property_bq_euclid_self_distance_zero` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. bq_len_bits_vs_bytes
- **Variant**: `bq_len_bits_vs_bytes_facc857_1`
- **Location**: `patches/bq_len_bits_vs_bytes_facc857_1.patch` (target `src/unaligned_vector/binary_quantized.rs`)
- **Property**: `property_bq_len_matches_iter`
- **Witness(es)**: `witness_bq_len_matches_iter_case_five_floats`
- **Fix commit**: `facc8575222d3f5da5b9a94288e44896911e701f` — fix the size of the iterator
- **Invariant violated**: `UnalignedVector<BinaryQuantized>::len()` must equal the number of items produced by `iter()` and `to_vec()` (ExactSizeIterator contract).
- **How the mutation triggers**: `len()` returns `vec.vector.len() / QUANTIZED_WORD_BYTES` (a word count) instead of `(vec.vector.len() / QUANTIZED_WORD_BYTES) * QUANTIZED_WORD_BITS` (a bit count). The iterator still yields `words * 64` f32s, so `len()` falls out of sync by a factor of 64.

### 2. cosine_distance_no_clamp
- **Variant**: `cosine_distance_no_clamp_6a09118_1`
- **Location**: `patches/cosine_distance_no_clamp_6a09118_1.patch` (target `src/distance/cosine.rs`)
- **Property**: `property_cosine_distance_in_unit`
- **Witness(es)**: `witness_cosine_distance_in_unit_case_identical_ones`
- **Fix commit**: `6a091180d7558d464cdd2ce9daec35e055e1d91c` — Numerical stability improvements for Cosine Distance
- **Invariant violated**: `Cosine::built_distance(p, q)` must be finite and within `[0.0, 1.0]` for any pair of sanitized, non-empty vectors.
- **How the mutation triggers**: The division guard is weakened from `pnqn > f32::EPSILON` to `pnqn != 0.0`, and the subsequent `cos.clamp(-1.0, 1.0)` is removed. In f32 `sqrt(7)^2 != 7`, so the cosine of seven-ones against itself can land slightly above 1.0, making the distance `1 - cos` negative.

### 3. bq_euclid_self_distance_or_not_xor
- **Variant**: `bq_euclid_self_distance_or_not_xor_47fd2bd_1`
- **Location**: `patches/bq_euclid_self_distance_or_not_xor_47fd2bd_1.patch` (target `src/distance/binary_quantized_euclidean.rs`)
- **Property**: `property_bq_euclid_self_distance_zero`
- **Witness(es)**: `witness_bq_euclid_self_distance_zero_case_small_positive`
- **Fix commit**: `47fd2bdd7f59efa84ddd9ea5dcb1b10c32b4849c` — fix the normalized distance for the binary quantized euclidean distance
- **Invariant violated**: `BinaryQuantizedEuclidean::built_distance(p, p)` must be `0.0` for every leaf (self-identity axiom of a distance metric).
- **How the mutation triggers**: The popcount argument changes from `u ^ v` (XOR, which gives `0` for `u == v`) to `u | v` (OR, which equals `popcount(v)` for `u == v`). Any leaf with at least one positive scalar then reports `built_distance(p, p) = 4 * popcount(v) > 0`.
