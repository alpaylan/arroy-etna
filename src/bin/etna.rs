// ETNA workload runner for arroy.
//
// Usage: cargo run --release --bin etna -- <tool> <property>
//   tool:     etna | proptest | quickcheck | crabcheck | hegel
//   property: BqLenMatchesIter
//           | CosineDistanceInUnit
//           | BqEuclidSelfDistanceZero
//           | All
//
// Each invocation emits a single JSON line on stdout and exits 0
// (usage errors exit 2).

use arroy::etna::{
    property_bq_euclid_self_distance_zero, property_bq_len_matches_iter,
    property_cosine_distance_in_unit, PropertyResult,
};
use crabcheck::quickcheck as crabcheck_qc;
use hegel::{generators as hgen, Hegel, Settings as HegelSettings};
use proptest::prelude::*;
use proptest::test_runner::{Config as ProptestConfig, TestCaseError, TestRunner};
use quickcheck::{QuickCheck, ResultStatus, TestResult};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Default, Clone, Copy)]
struct Metrics {
    inputs: u64,
    elapsed_us: u128,
}

impl Metrics {
    fn combine(self, other: Metrics) -> Metrics {
        Metrics {
            inputs: self.inputs + other.inputs,
            elapsed_us: self.elapsed_us + other.elapsed_us,
        }
    }
}

type Outcome = (Result<(), String>, Metrics);

fn to_err(r: PropertyResult) -> Result<(), String> {
    match r {
        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
        PropertyResult::Fail(m) => Err(m),
    }
}

const ALL_PROPERTIES: &[&str] = &[
    "BqLenMatchesIter",
    "CosineDistanceInUnit",
    "BqEuclidSelfDistanceZero",
];

fn run_all<F: FnMut(&str) -> Outcome>(mut f: F) -> Outcome {
    let mut total = Metrics::default();
    let mut final_status: Result<(), String> = Ok(());
    for p in ALL_PROPERTIES {
        let (r, m) = f(p);
        total = total.combine(m);
        if let Err(e) = r {
            if final_status.is_ok() {
                final_status = Err(e);
            }
        }
    }
    (final_status, total)
}

// ───────────── etna tool: replays frozen witness inputs. ─────────────
fn run_etna_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_etna_property);
    }
    let t0 = Instant::now();
    let result = match property {
        "BqLenMatchesIter" => to_err(property_bq_len_matches_iter(vec![
            -1.0, 2.0, -3.0, 4.0, 5.0,
        ])),
        "CosineDistanceInUnit" => {
            to_err(property_cosine_distance_in_unit(vec![1.0; 7], vec![1.0; 7]))
        }
        "BqEuclidSelfDistanceZero" => {
            to_err(property_bq_euclid_self_distance_zero(vec![1.0, -2.0, 3.0]))
        }
        _ => {
            return (
                Err(format!("Unknown property: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    (result, Metrics { inputs: 1, elapsed_us })
}

// ───────────── proptest ─────────────
fn float_strategy() -> BoxedStrategy<f32> {
    // Bound to finite, moderate-magnitude floats — the properties sanitize
    // but keeping the strategy tight means we exercise realistic values.
    (-100.0f32..=100.0).boxed()
}

fn float_vec_strategy() -> BoxedStrategy<Vec<f32>> {
    prop::collection::vec(float_strategy(), 1..=16).boxed()
}

// The cosine-distance bug needs near-parallel vectors (e.g. a==b of all-ones)
// to surface — pure random pairs don't hit the rounding edge. Mix independent
// pairs with identical pairs so the unclamped-cosine > 1.0 case gets sampled.
fn float_vec_pair_strategy() -> BoxedStrategy<(Vec<f32>, Vec<f32>)> {
    prop_oneof![
        (float_vec_strategy(), float_vec_strategy()),
        float_vec_strategy().prop_map(|v| (v.clone(), v)),
    ]
    .boxed()
}

fn run_proptest_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_proptest_property);
    }
    let counter = Arc::new(AtomicU64::new(0));
    let t0 = Instant::now();
    let mut runner = TestRunner::new(ProptestConfig::default());
    let result: Result<(), String> = match property {
        "BqLenMatchesIter" => {
            let c = counter.clone();
            runner
                .run(&float_vec_strategy(), move |v| {
                    c.fetch_add(1, Ordering::Relaxed);
                    match property_bq_len_matches_iter(v) {
                        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
                        PropertyResult::Fail(m) => Err(TestCaseError::fail(m)),
                    }
                })
                .map_err(|e| e.to_string())
        }
        "CosineDistanceInUnit" => {
            let c = counter.clone();
            runner
                .run(&float_vec_pair_strategy(), move |(a, b)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    match property_cosine_distance_in_unit(a, b) {
                        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
                        PropertyResult::Fail(m) => Err(TestCaseError::fail(m)),
                    }
                })
                .map_err(|e| e.to_string())
        }
        "BqEuclidSelfDistanceZero" => {
            let c = counter.clone();
            runner
                .run(&float_vec_strategy(), move |v| {
                    c.fetch_add(1, Ordering::Relaxed);
                    match property_bq_euclid_self_distance_zero(v) {
                        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
                        PropertyResult::Fail(m) => Err(TestCaseError::fail(m)),
                    }
                })
                .map_err(|e| e.to_string())
        }
        _ => {
            return (
                Err(format!("Unknown property for proptest: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = counter.load(Ordering::Relaxed);
    (result, Metrics { inputs, elapsed_us })
}

// ───────────── quickcheck (fork with `etna` feature) ─────────────
// The fork's Testable impl under `etna` requires Display on every argument.
// Vec<f32> lacks Display, so we drive each property from u64 seeds that we
// expand deterministically into the Vec<f32> the property_* functions expect.
static QC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn seed_to_f32_vec(seed: u64) -> Vec<f32> {
    let len = ((seed >> 60) as usize) % 16 + 1;
    let mut out = Vec::with_capacity(len);
    let mut s = seed;
    for _ in 0..len {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        // Map 64 bits into a moderate, finite f32 in roughly [-100, 100].
        let raw = (s >> 33) as u32;
        let f = ((raw as i32 as f32) / (i32::MAX as f32)) * 100.0;
        out.push(f);
    }
    out
}

fn qc_bq_len_matches_iter(seed: u64) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_bq_len_matches_iter(seed_to_f32_vec(seed)) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_cosine_distance_in_unit(a: u64, b: u64, identical: bool) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    // Force identical pairs on ~1/2 of calls so the rounding-driven bug
    // (unclamped cosine > 1.0 for near-parallel vectors) is sampled.
    let (va, vb) = if identical {
        let v = seed_to_f32_vec(a);
        (v.clone(), v)
    } else {
        (seed_to_f32_vec(a), seed_to_f32_vec(b))
    };
    match property_cosine_distance_in_unit(va, vb) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_bq_euclid_self_distance_zero(seed: u64) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_bq_euclid_self_distance_zero(seed_to_f32_vec(seed)) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn run_quickcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_quickcheck_property);
    }
    QC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let mut qc = QuickCheck::new().tests(200).max_tests(2000);
    let result = match property {
        "BqLenMatchesIter" => qc.quicktest(qc_bq_len_matches_iter as fn(u64) -> TestResult),
        "CosineDistanceInUnit" => {
            qc.quicktest(qc_cosine_distance_in_unit as fn(u64, u64, bool) -> TestResult)
        }
        "BqEuclidSelfDistanceZero" => {
            qc.quicktest(qc_bq_euclid_self_distance_zero as fn(u64) -> TestResult)
        }
        _ => {
            return (
                Err(format!("Unknown property for quickcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = QC_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match result.status {
        ResultStatus::Finished => Ok(()),
        ResultStatus::Failed { arguments } => Err(format!(
            "quickcheck counterexample: ({})",
            arguments.join(" ")
        )),
        ResultStatus::Aborted { err } => Err(format!("quickcheck aborted: {err:?}")),
        ResultStatus::TimedOut => Err("quickcheck timed out".into()),
        ResultStatus::GaveUp => Err(format!(
            "quickcheck gave up: passed={}, discarded={}",
            result.n_tests_passed, result.n_tests_discarded
        )),
    };
    (status, metrics)
}

// ───────────── crabcheck ─────────────
static CC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn usize_vec_to_f32_vec(v: Vec<usize>) -> Vec<f32> {
    // Crabcheck's Arbitrary<usize> yields small non-negative integers; scale
    // to a signed, moderate range so both bugs have a chance to surface.
    v.into_iter()
        .map(|x| {
            let h = (x as u32).wrapping_mul(2654435761);
            let signed = h as i32;
            (signed as f32 / (i32::MAX as f32)) * 100.0
        })
        .collect()
}

fn cc_bq_len_matches_iter(v: Vec<usize>) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_bq_len_matches_iter(usize_vec_to_f32_vec(v)) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_cosine_distance_in_unit((a, b, identical): (Vec<usize>, Vec<usize>, bool)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    // With probability ~1/2 we force a == b so the near-parallel rounding case
    // (required to surface the unclamped-cosine bug) is actually sampled.
    let (va, vb) = if identical {
        let v = usize_vec_to_f32_vec(a);
        (v.clone(), v)
    } else {
        (usize_vec_to_f32_vec(a), usize_vec_to_f32_vec(b))
    };
    match property_cosine_distance_in_unit(va, vb) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_bq_euclid_self_distance_zero(v: Vec<usize>) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_bq_euclid_self_distance_zero(usize_vec_to_f32_vec(v)) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn run_crabcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_crabcheck_property);
    }
    CC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let result = match property {
        "BqLenMatchesIter" => crabcheck_qc::quickcheck(cc_bq_len_matches_iter),
        "CosineDistanceInUnit" => crabcheck_qc::quickcheck(cc_cosine_distance_in_unit),
        "BqEuclidSelfDistanceZero" => crabcheck_qc::quickcheck(cc_bq_euclid_self_distance_zero),
        _ => {
            return (
                Err(format!("Unknown property for crabcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = CC_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match result.status {
        crabcheck_qc::ResultStatus::Finished => Ok(()),
        crabcheck_qc::ResultStatus::Failed { arguments } => Err(format!(
            "crabcheck counterexample: ({})",
            arguments.join(" ")
        )),
        crabcheck_qc::ResultStatus::TimedOut => Err("crabcheck timed out".into()),
        crabcheck_qc::ResultStatus::GaveUp => Err(format!(
            "crabcheck gave up: passed={}, discarded={}",
            result.passed, result.discarded
        )),
        crabcheck_qc::ResultStatus::Aborted { error } => Err(format!("crabcheck aborted: {error}")),
    };
    (status, metrics)
}

// ───────────── hegel (hegeltest 0.3.7) ─────────────
static HG_COUNTER: AtomicU64 = AtomicU64::new(0);

fn hegel_settings() -> HegelSettings {
    HegelSettings::new().test_cases(200).seed(Some(0x0A12_0044))
}

fn draw_f32_vec(tc: &hegel::TestCase, max_len: usize) -> Vec<f32> {
    // Draw a usize-ish length and per-element raw bytes, then map into a
    // moderate signed f32. Hegel's generator library is integer-centric, so
    // we build f32s from the integer draws.
    let ints = tc.draw(hgen::vecs(hgen::integers::<i32>()).max_size(max_len));
    ints.into_iter()
        .map(|x| (x as f32 / (i32::MAX as f32)) * 100.0)
        .collect()
}

fn run_hegel_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_hegel_property);
    }
    HG_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let settings = hegel_settings();
    let run_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match property {
        "BqLenMatchesIter" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let v = draw_f32_vec(&tc, 16);
                if let PropertyResult::Fail(m) = property_bq_len_matches_iter(v) {
                    panic!("{m}");
                }
            })
            .settings(settings.clone())
            .run();
        }
        "CosineDistanceInUnit" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let a = draw_f32_vec(&tc, 16);
                let b = draw_f32_vec(&tc, 16);
                if let PropertyResult::Fail(m) = property_cosine_distance_in_unit(a, b) {
                    panic!("{m}");
                }
            })
            .settings(settings.clone())
            .run();
        }
        "BqEuclidSelfDistanceZero" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let v = draw_f32_vec(&tc, 16);
                if let PropertyResult::Fail(m) = property_bq_euclid_self_distance_zero(v) {
                    panic!("{m}");
                }
            })
            .settings(settings.clone())
            .run();
        }
        _ => panic!("__unknown_property:{property}"),
    }));
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = HG_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match run_result {
        Ok(()) => Ok(()),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "hegel panicked with non-string payload".to_string()
            };
            if let Some(rest) = msg.strip_prefix("__unknown_property:") {
                return (
                    Err(format!("Unknown property for hegel: {rest}")),
                    Metrics::default(),
                );
            }
            Err(format!("hegel found counterexample: {msg}"))
        }
    };
    (status, metrics)
}

fn run(tool: &str, property: &str) -> Outcome {
    match tool {
        "etna" => run_etna_property(property),
        "proptest" => run_proptest_property(property),
        "quickcheck" => run_quickcheck_property(property),
        "crabcheck" => run_crabcheck_property(property),
        "hegel" => run_hegel_property(property),
        _ => (Err(format!("Unknown tool: {tool}")), Metrics::default()),
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit_json(
    tool: &str,
    property: &str,
    status: &str,
    metrics: Metrics,
    counterexample: Option<&str>,
    error: Option<&str>,
) {
    let cex = counterexample.map_or("null".to_string(), json_str);
    let err = error.map_or("null".to_string(), json_str);
    println!(
        "{{\"status\":{},\"tests\":{},\"discards\":0,\"time\":{},\"counterexample\":{},\"error\":{},\"tool\":{},\"property\":{}}}",
        json_str(status),
        metrics.inputs,
        json_str(&format!("{}us", metrics.elapsed_us)),
        cex,
        err,
        json_str(tool),
        json_str(property),
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property>", args[0]);
        eprintln!("Tools: etna | proptest | quickcheck | crabcheck | hegel");
        eprintln!("Properties: BqLenMatchesIter | CosineDistanceInUnit | BqEuclidSelfDistanceZero | All");
        std::process::exit(2);
    }
    let (tool, property) = (args[1].as_str(), args[2].as_str());

    // Silence library-under-test panic noise (frameworks catch panics internally
    // but the default hook still prints "thread 'main' panicked at ..." to stderr).
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(tool, property)));
    std::panic::set_hook(previous_hook);

    let (result, metrics) = match caught {
        Ok(outcome) => outcome,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "panic with non-string payload".to_string()
            };
            emit_json(
                tool,
                property,
                "aborted",
                Metrics::default(),
                None,
                Some(&format!("adapter panic: {msg}")),
            );
            return;
        }
    };

    match result {
        Ok(()) => emit_json(tool, property, "passed", metrics, None, None),
        Err(msg) => emit_json(tool, property, "failed", metrics, Some(&msg), None),
    }
}
