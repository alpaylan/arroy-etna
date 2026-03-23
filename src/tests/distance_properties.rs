use proptest::collection::vec;
use proptest::prelude::*;

use crate::distances::Euclidean;
use crate::internals::{Leaf, Side, UnalignedVector};
use crate::Distance;

fn euclidean_leaf(values: Vec<f32>) -> Leaf<'static, Euclidean> {
    let vector = UnalignedVector::from_vec(values);
    let header = Euclidean::new_header(vector.as_ref());
    Leaf { header, vector }
}

proptest! {
    #[test]
    fn normalized_distance_matches_square_root(distance in 0.0f32..10_000.0f32) {
        let got = Euclidean::normalized_distance(distance, 8);
        let expected = distance.sqrt();
        let tol = 1e-6 * expected.max(1.0);
        prop_assert!((got - expected).abs() <= tol);
    }

    #[test]
    fn pq_left_side_uses_negative_margin(
        distance in 0.0f32..10_000.0f32,
        margin in 0.001f32..10_000.0f32,
    ) {
        let left = Euclidean::pq_distance(distance, margin, Side::Left);
        let right = Euclidean::pq_distance(distance, margin, Side::Right);
        prop_assert!(left <= 0.0);
        prop_assert!(left <= right + 1e-6);
    }

    #[test]
    fn update_mean_respects_weighted_average_divisor(
        pairs in vec((-100.0f32..100.0f32, -100.0f32..100.0f32), 1..24),
        c in 1.0f32..20.0f32,
        norm in 0.1f32..20.0f32,
    ) {
        let base: Vec<f32> = pairs.iter().map(|(x, _)| *x).collect();
        let incoming: Vec<f32> = pairs.iter().map(|(_, n)| *n).collect();

        let mut mean = euclidean_leaf(base.clone());
        let new_node = euclidean_leaf(incoming.clone());
        Euclidean::update_mean(&mut mean, &new_node, norm, c);

        let got: Vec<f32> = mean.vector.iter().collect();
        for i in 0..pairs.len() {
            let expected = (base[i] * c + incoming[i] / norm) / (c + 1.0);
            let tol = 1e-5 * expected.abs().max(1.0);
            prop_assert!((got[i] - expected).abs() <= tol);
        }
    }

    #[test]
    fn side_is_consistent_with_margin_sign(
        normal in vec(-10.0f32..10.0f32, 1..16),
        node in vec(-10.0f32..10.0f32, 1..16),
    ) {
        let len = normal.len().min(node.len());
        prop_assume!(len > 0);

        let normal = euclidean_leaf(normal.into_iter().take(len).collect());
        let node = euclidean_leaf(node.into_iter().take(len).collect());
        let margin = Euclidean::margin(&normal, &node);
        prop_assume!(margin.abs() > 1e-4);

        let side = Euclidean::side(&normal, &node);
        if margin.is_sign_positive() {
            prop_assert!(matches!(side, Side::Right));
        } else {
            prop_assert!(matches!(side, Side::Left));
        }
    }
}
