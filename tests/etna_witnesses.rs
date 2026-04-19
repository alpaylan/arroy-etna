//! Deterministic witness tests for arroy ETNA variants.
//!
//! Each `witness_<name>_case_<tag>` passes on the base HEAD and fails under
//! the corresponding `etna/<variant>` branch. Witnesses call `property_<name>`
//! directly with frozen inputs — no proptest/quickcheck/RNG/clock machinery.

use arroy::etna::{
    property_bq_euclid_self_distance_zero, property_bq_len_matches_iter,
    property_cosine_distance_in_unit, PropertyResult,
};

fn expect_pass(r: PropertyResult, what: &str) {
    match r {
        PropertyResult::Pass => {}
        PropertyResult::Fail(m) => panic!("{what}: property failed: {m}"),
        PropertyResult::Discard => panic!("{what}: unexpected discard"),
    }
}

// Variant: bq_len_bits_vs_bytes_facc857_1
// Five-element f32 slice quantizes to one 64-bit word (eight bytes). The
// iterator yields 64 f32 values; `len()` on the base returns 64. Under the
// bug, `len()` returned `bytes / 8 = 1`, but the iterator still yielded 64
// items — the witness asserts these stay consistent.
#[test]
fn witness_bq_len_matches_iter_case_five_floats() {
    expect_pass(
        property_bq_len_matches_iter(vec![-1.0, 2.0, -3.0, 4.0, 5.0]),
        "bq_len_matches_iter / five_floats",
    );
}

// Variant: cosine_distance_no_clamp_6a09118_1
// A vector of seven ones against itself. In exact arithmetic the cosine is
// 1.0 and the distance is 0.0. In f32, `sqrt(7)^2 != 7`, so the bug-version
// divides 7.0 by a slightly-different value and the cosine falls either side
// of 1.0. The base clamps to [-1, 1] first, so the distance stays in [0, 1];
// the buggy version allows cos > 1 → negative distance.
#[test]
fn witness_cosine_distance_in_unit_case_identical_ones() {
    expect_pass(
        property_cosine_distance_in_unit(vec![1.0; 7], vec![1.0; 7]),
        "cosine_distance_in_unit / identical_ones",
    );
}

// Variant: bq_euclid_self_distance_or_not_xor_47fd2bd_1
// Self-distance of a three-scalar binary-quantized vector with some positive
// bits. On the base, built_distance(p, p) = 0. With the bug (OR-popcount
// instead of XOR-popcount), it equals 4 * popcount(v) > 0.
#[test]
fn witness_bq_euclid_self_distance_zero_case_small_positive() {
    expect_pass(
        property_bq_euclid_self_distance_zero(vec![1.0, -2.0, 3.0]),
        "bq_euclid_self_distance_zero / small_positive",
    );
}
