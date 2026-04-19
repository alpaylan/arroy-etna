//! ETNA framework-neutral property functions for arroy.
//!
//! Each `property_<name>` is a pure function taking concrete, owned inputs and
//! returning `PropertyResult`. Framework adapters (proptest/quickcheck/crabcheck/hegel)
//! in `src/bin/etna.rs` and deterministic witness tests in `tests/etna_witnesses.rs`
//! both call these functions directly.

#![allow(missing_docs)]

use crate::distance::{BinaryQuantizedEuclidean, Cosine, Distance};
use crate::internals::{Leaf, UnalignedVector, UnalignedVectorCodec};

pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

// ──────────────────────────────────────────────────────────────────────────
// Property 1: BinaryQuantized::len reports bits-count (consistent with the
// iterator), not bytes-count.
//
// Regression for commit facc8575 ("fix the size of the iterator"). The buggy
// version returned `vec.vector.len() / QUANTIZED_WORD_BYTES`, i.e. the number
// of words, while the iterator yields `words * QUANTIZED_WORD_BITS` f32
// values. The ExactSizeIterator contract requires `len` to match the number
// of items produced by `iter()` and `to_vec()`.
// ──────────────────────────────────────────────────────────────────────────
pub fn property_bq_len_matches_iter(original: Vec<f32>) -> PropertyResult {
    use crate::distance::BinaryQuantizedEuclidean;

    if original.is_empty() {
        return PropertyResult::Discard;
    }
    // Cap input length so property runs fast under every framework.
    let original: Vec<f32> = original.into_iter().take(512).collect();

    // UnalignedVector<BinaryQuantized> is produced via Distance::Header::VectorCodec.
    // We go through the library's public from_slice to exercise the same code path.
    // NOTE: deliberately skip `to_vec()` here — the buggy variant reports a truncated
    // `len()` and the SSE/NEON implementations pre-allocate `vec![0.0; vec.len()]` before
    // writing an iter's worth of f32s into it. A mismatched length corrupts memory and
    // aborts with SIGABRT (uncatchable by `catch_unwind`). The ExactSizeIterator contract
    // we care about here is `len() == iter().count()`, which is enough to detect the
    // bug without tripping the UB path.
    let cow = <BinaryQuantizedEuclidean as Distance>::VectorCodec::from_slice(&original);
    let vec_ref: &UnalignedVector<_> = &cow;
    let reported = vec_ref.len();
    let iter_count = vec_ref.iter().count();

    if reported != iter_count {
        return PropertyResult::Fail(format!(
            "len()={} but iter().count()={} (input len={})",
            reported,
            iter_count,
            original.len()
        ));
    }
    PropertyResult::Pass
}

// ──────────────────────────────────────────────────────────────────────────
// Property 2: Cosine::built_distance is always in [0.0, 1.0] and finite.
//
// Regression for commit 6a091180 ("Numerical stability improvements for
// Cosine Distance"). The buggy version divides by `pn * qn` whenever it is
// nonzero and never clamps the resulting cosine. For near-parallel vectors,
// sqrt-then-multiply loses precision so cos can be slightly > 1.0, yielding
// negative distances. For very-small-norm vectors, the division may produce
// noisy large values, pushing the distance far outside [0, 1].
// ──────────────────────────────────────────────────────────────────────────
fn build_cosine_leaf(vec: Vec<f32>) -> Leaf<'static, Cosine> {
    let vector = UnalignedVector::<f32>::from_vec(vec);
    let header = <Cosine as Distance>::new_header(&vector);
    Leaf { header, vector }
}

fn sanitize_float(x: f32) -> f32 {
    if !x.is_finite() {
        0.0
    } else {
        x.clamp(-1_000.0, 1_000.0)
    }
}

pub fn property_cosine_distance_in_unit(a: Vec<f32>, b: Vec<f32>) -> PropertyResult {
    // Equal length, bounded length, bounded values, no NaN/Inf.
    if a.is_empty() || b.is_empty() {
        return PropertyResult::Discard;
    }
    let n = std::cmp::min(a.len(), b.len()).min(16);
    if n == 0 {
        return PropertyResult::Discard;
    }
    let va: Vec<f32> = a.iter().take(n).copied().map(sanitize_float).collect();
    let vb: Vec<f32> = b.iter().take(n).copied().map(sanitize_float).collect();

    let p = build_cosine_leaf(va.clone());
    let q = build_cosine_leaf(vb.clone());
    let d = <Cosine as Distance>::built_distance(&p, &q);

    if !d.is_finite() {
        return PropertyResult::Fail(format!(
            "built_distance is non-finite: d={} (a={:?}, b={:?})",
            d, va, vb
        ));
    }
    if d < 0.0 || d > 1.0 {
        return PropertyResult::Fail(format!(
            "built_distance={} outside [0, 1] (a={:?}, b={:?})",
            d, va, vb
        ));
    }
    PropertyResult::Pass
}

// ──────────────────────────────────────────────────────────────────────────
// Property 3: BinaryQuantizedEuclidean::built_distance(p, p) == 0.
//
// Regression for commit 47fd2bdd ("fix the normalized distance for the binary
// quantized euclidean distance"). The buggy version computed distance via
// popcount(u OR v); for any leaf containing a positive scalar, popcount(v|v)
// equals popcount(v) > 0, violating the self-identity axiom of a distance
// metric (d(p, p) must be 0). The fix swaps OR for XOR (squared-Euclidean on
// ±1 bits), giving popcount(v^v) = 0.
// ──────────────────────────────────────────────────────────────────────────
fn build_bqeuclid_leaf(vec: Vec<f32>) -> Leaf<'static, BinaryQuantizedEuclidean> {
    let vector = <BinaryQuantizedEuclidean as Distance>::VectorCodec::from_vec(vec);
    let header = <BinaryQuantizedEuclidean as Distance>::new_header(&vector);
    Leaf { header, vector }
}

pub fn property_bq_euclid_self_distance_zero(v: Vec<f32>) -> PropertyResult {
    if v.is_empty() {
        return PropertyResult::Discard;
    }
    // Cap input length to stay fast across frameworks.
    let v: Vec<f32> = v.into_iter().take(256).map(sanitize_float).collect();
    // Binary-quantized vectors map every scalar to +1 or -1 by its sign. If
    // every scalar is zero, the quantized vector is all-zero bits and the
    // property trivially holds on both base and variant — discard to avoid
    // a false pass.
    let any_positive = v.iter().any(|x| x.is_sign_positive());
    if !any_positive {
        return PropertyResult::Discard;
    }

    let p = build_bqeuclid_leaf(v.clone());
    let q = build_bqeuclid_leaf(v.clone());
    let d = <BinaryQuantizedEuclidean as Distance>::built_distance(&p, &q);

    if d != 0.0 {
        return PropertyResult::Fail(format!(
            "built_distance(p, p) = {} (expected 0.0) for v.len()={}",
            d,
            v.len()
        ));
    }
    PropertyResult::Pass
}
