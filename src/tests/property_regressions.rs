use std::borrow::Cow;
use std::num::NonZeroUsize;

use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use roaring::RoaringBitmap;

use super::create_database;
use crate::distances::{Cosine, Euclidean};
use crate::node::{Descendants, Node};
use crate::parallel::TmpNodes;
use crate::writer::{fit_in_memory, target_n_trees, BuildOption};
use crate::{Reader, Writer};

fn build_random_euclidean_db(
    seed: u64,
    len: usize,
    dims: usize,
    n_trees: usize,
) -> (super::DatabaseHandle<Euclidean>, Vec<Vec<f32>>) {
    let handle = create_database::<Euclidean>();
    let mut vectors = Vec::with_capacity(len);
    let mut rng = StdRng::seed_from_u64(seed);

    let mut wtxn = handle.env.write_txn().unwrap();
    let writer = Writer::new(handle.database, 0, dims);
    for id in 0..len as u32 {
        let vec: Vec<f32> = (0..dims).map(|_| rng.gen_range(-10.0f32..10.0)).collect();
        writer.add_item(&mut wtxn, id, &vec).unwrap();
        vectors.push(vec);
    }
    writer.builder(&mut rng).n_trees(n_trees).build(&mut wtxn).unwrap();
    wtxn.commit().unwrap();

    (handle, vectors)
}

fn expected_target_n_trees(dimensions: u64, nb_vec: u64) -> u64 {
    let nb_vec_f = nb_vec as f64;
    let dim_bonus = (768.0 / dimensions as f64).powf(4.0);
    let small_dataset_exponent = nb_vec_f.log2() - 6.0;
    let nb_trees = if nb_vec_f < 10_000.0 {
        2.0_f64.powf(small_dataset_exponent)
    } else {
        2.0_f64.powf(nb_vec_f.log10() + (dimensions as f64).log10() + dim_bonus)
    };
    nb_trees.ceil() as u64
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn target_n_trees_matches_formula(
        dimensions in 1u64..4096,
        items in 1u64..40_000,
    ) {
        let options = BuildOption::default();
        let bitmap = RoaringBitmap::from_sorted_iter(0..items as u32).unwrap();
        let got = target_n_trees(&options, dimensions, &bitmap, &[]);
        let expected = expected_target_n_trees(dimensions, items);
        prop_assert_eq!(got, expected);
    }

    #[test]
    fn fit_in_memory_sampling_invariants(
        seed in any::<u64>(),
        dimensions in 1usize..64,
        len in 16usize..256,
    ) {
        prop_assume!(len > dimensions + 2);
        let mut rng = StdRng::seed_from_u64(seed);
        let mut to_insert = RoaringBitmap::from_sorted_iter(0..len as u32).unwrap();
        let before = to_insert.clone();

        let sampled = fit_in_memory::<Euclidean, _>(1, &mut to_insert, dimensions, &mut rng).unwrap();

        prop_assert!(sampled.len() as usize >= dimensions + 1);
        prop_assert!(sampled.is_disjoint(&to_insert));
        prop_assert_eq!(sampled.len() + to_insert.len(), before.len());
        prop_assert_eq!(sampled | to_insert, before);
    }

    #[test]
    fn reader_candidates_singleton_is_respected(
        seed in any::<u64>(),
        len in 3usize..48,
        dims in 2usize..10,
        k in 1usize..12,
    ) {
        let (handle, _vectors) = build_random_euclidean_db(seed, len, dims, 8);

        let query_id = (seed as usize % len) as u32;
        let target_id = ((seed.rotate_left(11) as usize) % len) as u32;

        let rtxn = handle.env.read_txn().unwrap();
        let reader = Reader::<Euclidean>::open(&rtxn, 0, handle.database).unwrap();
        let candidates = RoaringBitmap::from_sorted_iter(Some(target_id)).unwrap();
        let mut query = reader.nns(k.min(len));
        query
            .candidates(&candidates)
            .search_k(NonZeroUsize::new(usize::MAX).unwrap());
        let out = query.by_item(&rtxn, query_id).unwrap().unwrap();

        prop_assert_eq!(out.len(), 1);
        prop_assert_eq!(out[0].0, target_id);
    }

    #[test]
    fn reader_full_scan_without_candidates_returns_all_items(
        seed in any::<u64>(),
        len in 2usize..24,
    ) {
        let dims = len + 1;
        let (handle, _vectors) = build_random_euclidean_db(seed, len, dims, 1);

        let query_id = (seed as usize % len) as u32;
        let rtxn = handle.env.read_txn().unwrap();
        let reader = Reader::<Euclidean>::open(&rtxn, 0, handle.database).unwrap();

        let mut query = reader.nns(len);
        query.search_k(NonZeroUsize::new(usize::MAX).unwrap());
        let out = query.by_item(&rtxn, query_id).unwrap().unwrap();

        prop_assert_eq!(out.len(), len);
    }

    #[test]
    fn reader_default_search_k_matches_explicit_root_budget(seed in any::<u64>()) {
        let len = 120usize;
        let dims = 2usize;

        let handle = create_database::<Euclidean>();
        let mut wtxn = handle.env.write_txn().unwrap();
        let writer = Writer::new(handle.database, 0, dims);
        for i in 0..len as u32 {
            let x = i as f32 + (seed as f32 * 0.0001);
            writer.add_item(&mut wtxn, i, &[x, 0.0]).unwrap();
        }
        let mut rng = StdRng::seed_from_u64(seed);
        writer.builder(&mut rng).n_trees(24).build(&mut wtxn).unwrap();
        wtxn.commit().unwrap();

        let rtxn = handle.env.read_txn().unwrap();
        let reader = Reader::<Euclidean>::open(&rtxn, 0, handle.database).unwrap();
        let count = 12usize;
        let explicit_k = count * reader.n_trees();

        let default = reader.nns(count).by_item(&rtxn, 0).unwrap().unwrap();
        let mut explicit = reader.nns(count);
        explicit.search_k(NonZeroUsize::new(explicit_k).unwrap());
        let explicit = explicit.by_item(&rtxn, 0).unwrap().unwrap();

        prop_assert_eq!(default, explicit);
    }

    #[test]
    fn reader_output_is_normalized_euclidean_distance(
        seed in any::<u64>(),
        len in 3usize..40,
        dims in 2usize..10,
        k in 1usize..16,
    ) {
        let (handle, vectors) = build_random_euclidean_db(seed, len, dims, 10);

        let query_id = (seed as usize % len) as u32;
        let rtxn = handle.env.read_txn().unwrap();
        let reader = Reader::<Euclidean>::open(&rtxn, 0, handle.database).unwrap();
        let mut query = reader.nns(k.min(len));
        query.search_k(NonZeroUsize::new(usize::MAX).unwrap());
        let out = query.by_item(&rtxn, query_id).unwrap().unwrap();

        let query_vec = &vectors[query_id as usize];
        for (item_id, got) in out {
            let item_vec = &vectors[item_id as usize];
            let expected = query_vec
                .iter()
                .zip(item_vec.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f32>()
                .sqrt();
            let tol = 1e-4 * expected.max(1.0);
            prop_assert!((got - expected).abs() <= tol);
        }
    }

    #[test]
    fn tmp_nodes_random_read_append_preserves_previous_nodes(
        seed in any::<u64>(),
        initial in 16u32..256u32,
        appended in 4u32..64u32,
    ) {
        let mut tmp_nodes = TmpNodes::<Cosine>::new().unwrap();
        let mut rng = StdRng::seed_from_u64(seed);

        let mk_desc = |start: u32| {
            Node::Descendants(Descendants {
                descendants: Cow::Owned(RoaringBitmap::from_iter(start..start + 8)),
            })
        };

        for i in 0..initial {
            tmp_nodes.put(i, &mk_desc(i * 10)).unwrap();
        }

        for _ in 0..(initial * 2) {
            let id = rng.gen_range(0..initial);
            let node = tmp_nodes.get(id).unwrap().unwrap();
            let Node::Descendants(Descendants { descendants }) = node else { unreachable!() };
            prop_assert_eq!(descendants.len(), 8);
            prop_assert!(descendants.contains(id * 10));
            prop_assert!(descendants.contains(id * 10 + 7));
        }

        for i in 0..appended {
            let id = initial + i;
            tmp_nodes.put(id, &mk_desc(id * 10)).unwrap();
        }

        for id in 0..initial {
            let node = tmp_nodes.get(id).unwrap().unwrap();
            let Node::Descendants(Descendants { descendants }) = node else { unreachable!() };
            prop_assert_eq!(descendants.len(), 8);
            prop_assert!(descendants.contains(id * 10));
            prop_assert!(descendants.contains(id * 10 + 7));
        }
    }
}
