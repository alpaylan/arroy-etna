# arroy Injected Bugs

This workload contains **45** detected bugs derived from historical fixes in [meilisearch/arroy](https://github.com/meilisearch/arroy).

Each bug is injected with [marauders](https://github.com/akeles/marauders) comment syntax and has one active variant (`M_<variant>=active`).
The base configuration is expected to pass (`passed=True`, duration ~5.04s in the latest run).

## How To Inspect

```sh
# list all mutations
marauders list --path workloads/Rust/arroy

# run baseline
python3 -m wkgen test base --workload-dir workloads/Rust/arroy --timeout 900

# run one bug variant
python3 -m wkgen test variant --workload-dir workloads/Rust/arroy --variant <variant_name> --timeout 900
```

## Bug Index

| # | Name | Variant | File | Type | Failing Tests | Fix Commit |
|---|---|---|---|---|---:|---|
| 1 | `tmpfile_missing_seek` | `tmpfile_missing_seek_1` | `src/parallel.rs:41` | `missing-seek` | 1 | [`a63f097`](https://github.com/meilisearch/arroy/commit/a63f0979b216dde10d50fdfa4fadcb2b1dea73c7) |
| 2 | `reader_empty_search_guard` | `reader_empty_search_guard_1` | `src/reader.rs:323` | `missing-base-case` | 2 | [`2128749`](https://github.com/meilisearch/arroy/commit/2128749806c67ca8853395e34bf0b4df1f9e41bc) |
| 3 | `tree_target_rounding` | `tree_target_rounding_1` | `src/writer.rs:1423` | `wrong-rounding` | 2 | [`5b6f5b8`](https://github.com/meilisearch/arroy/commit/5b6f5b8de08e87680ad0ac0049602d43717b5a93) |
| 4 | `fit_in_memory_remove_by_rank` | `fit_in_memory_remove_by_rank_1` | `src/writer.rs:1672` | `wrong-remove-operation` | 3 | [`5262b08`](https://github.com/meilisearch/arroy/commit/5262b0866ed6a0b8fe529b170a1700fe4a47ebbd) |
| 5 | `low_memory_none_handling` | `low_memory_none_handling_1` | `src/writer.rs:1560` | `missing-none-handling` | 1 | [`af2cdca`](https://github.com/meilisearch/arroy/commit/af2cdcae43ab5ca6b77ad6cad50df5e64af43b82) |
| 6 | `upgrade_wrong_metadata_key` | `upgrade_wrong_metadata_key_1` | `src/upgrade.rs:111` | `wrong-key-field` | 1 | [`f3db126`](https://github.com/meilisearch/arroy/commit/f3db126a274264e90bb0eae70892da8293d74425) |
| 7 | `upgrade_splitnode_rewrite_gate` | `upgrade_splitnode_rewrite_gate_1` | `src/upgrade.rs:287` | `conditional-rewrite` | 2 | [`7639506`](https://github.com/meilisearch/arroy/commit/7639506680d32853e1dc825a1d99ea974a4b1bb7) |
| 8 | `tmp_lookup_search_window` | `tmp_lookup_search_window_1` | `src/parallel.rs:161` | `narrow-search-window` | 2 | [`c0e66c6`](https://github.com/meilisearch/arroy/commit/c0e66c674c210ad8422f4f3bf6e043a729f5d914) |
| 9 | `fit_in_memory_min_split_threshold` | `fit_in_memory_min_split_threshold_1` | `src/writer.rs:1655` | `off-by-one-split-threshold` | 0 | [`fa62650`](https://github.com/meilisearch/arroy/commit/fa626508854b33d0e6994ba601bf27d9c1315eea) |
| 10 | `missing_tree_count_direction` | `missing_tree_count_direction_1` | `src/writer.rs:551` | `wrong-missing-tree-count` | 13 | [`eaeb864`](https://github.com/meilisearch/arroy/commit/eaeb864f8f9250893fd6fd110729d7dabae38c03) |
| 11 | `extraneous_tree_count_direction` | `extraneous_tree_count_direction_1` | `src/writer.rs:652` | `wrong-extraneous-tree-count` | 3 | [`66068b4`](https://github.com/meilisearch/arroy/commit/66068b485aad6bd7c5a720325b9237c32a79c132) |
| 12 | `delete_empty_left_child_promote` | `delete_empty_left_child_promote_1` | `src/writer.rs:1083` | `stale-empty-child-promotion` | 1 | [`c6c53c5`](https://github.com/meilisearch/arroy/commit/c6c53c5630e5db417492147e4533908abd05479a) |
| 13 | `fit_descendant_strict_threshold` | `fit_descendant_strict_threshold_1` | `src/writer.rs:476` | `strict-fit-threshold` | 15 | [`dfc3e60`](https://github.com/meilisearch/arroy/commit/dfc3e600d35bacb937e3d4e170525ab03d74ecb6) |
| 14 | `single_leaf_root_id_mismatch` | `single_leaf_root_id_mismatch_1` | `src/writer.rs:957` | `wrong-single-leaf-root-id` | 18 | [`dfc3e60`](https://github.com/meilisearch/arroy/commit/dfc3e600d35bacb937e3d4e170525ab03d74ecb6) |
| 15 | `tree_target_small_dataset_boundary` | `tree_target_small_dataset_boundary_1` | `src/writer.rs:1415` | `small-dataset-threshold` | 1 | [`92c1ddd`](https://github.com/meilisearch/arroy/commit/92c1dddd2d59f6d7a7a4069c7f746d963d9dd08a) |
| 16 | `tree_target_dimension_power` | `tree_target_dimension_power_1` | `src/writer.rs:1401` | `dimension-scaling-power` | 1 | [`92c1ddd`](https://github.com/meilisearch/arroy/commit/92c1dddd2d59f6d7a7a4069c7f746d963d9dd08a) |
| 17 | `euclidean_split_skip_normalize` | `euclidean_split_skip_normalize_1` | `src/distance/euclidean.rs:84` | `missing-split-normalization` | 10 | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| 18 | `median_top_k_truncate_bias` | `median_top_k_truncate_bias_1` | `src/reader.rs:707` | `early-buffer-truncation` | 2 | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| 19 | `euclidean_split_bias_scale` | `euclidean_split_bias_scale_1` | `src/distance/euclidean.rs:99` | `wrong-split-bias-scale` | 10 | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| 20 | `median_top_k_final_truncate` | `median_top_k_final_truncate_1` | `src/reader.rs:732` | `off-by-one-final-truncate` | 7 | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| 21 | `euclidean_margin_sign` | `euclidean_margin_sign_1` | `src/distance/euclidean.rs:111` | `inverted-margin-sign` | 11 | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| 22 | `median_top_k_filter_inversion` | `median_top_k_filter_inversion_1` | `src/reader.rs:693` | `inverted-threshold-filter` | 2 | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| 23 | `median_top_k_sort_direction` | `median_top_k_sort_direction_1` | `src/reader.rs:725` | `reversed-ordering` | 6 | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| 24 | `reader_searchk_root_factor` | `reader_searchk_root_factor_1` | `src/reader.rs:335` | `missing-root-factor` | 3 | [`3538b81`](https://github.com/meilisearch/arroy/commit/3538b817105bac074f601008fbaba594eb5d121a) |
| 25 | `reader_candidates_filter_bypass` | `reader_candidates_filter_bypass_1` | `src/reader.rs:374` | `candidate-filter-bypass` | 1 | [`3e7d3d0`](https://github.com/meilisearch/arroy/commit/3e7d3d0926e26d60e16f0f10ddf82596f782670b) |
| 26 | `reader_margin_argument_order` | `reader_margin_argument_order_1` | `src/reader.rs:399` | `swapped-margin-arguments` | 1 | [`3538b81`](https://github.com/meilisearch/arroy/commit/3538b817105bac074f601008fbaba594eb5d121a) |
| 27 | `reader_split_side_distance_swap` | `reader_split_side_distance_swap_1` | `src/reader.rs:412` | `swapped-branch-priority` | 1 | [`3538b81`](https://github.com/meilisearch/arroy/commit/3538b817105bac074f601008fbaba594eb5d121a) |
| 28 | `reader_nns_dedup_order` | `reader_nns_dedup_order_1` | `src/reader.rs:427` | `dedup-before-sort` | 3 | [`3538b81`](https://github.com/meilisearch/arroy/commit/3538b817105bac074f601008fbaba594eb5d121a) |
| 29 | `median_top_k_initial_threshold` | `median_top_k_initial_threshold_1` | `src/reader.rs:679` | `wrong-threshold-initialization` | 2 | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| 30 | `reader_leaf_candidates_default_false` | `reader_leaf_candidates_default_false_1` | `src/reader.rs:361` | `wrong-candidate-default` | 1 | [`3e7d3d0`](https://github.com/meilisearch/arroy/commit/3e7d3d0926e26d60e16f0f10ddf82596f782670b) |
| 31 | `reader_descendant_no_candidates_skip` | `reader_descendant_no_candidates_skip_1` | `src/reader.rs:382` | `skip-descendants-without-candidates` | 5 | [`3e7d3d0`](https://github.com/meilisearch/arroy/commit/3e7d3d0926e26d60e16f0f10ddf82596f782670b) |
| 32 | `reader_k_selection_max` | `reader_k_selection_max_1` | `src/reader.rs:450` | `max-instead-of-min-top-k` | 4 | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| 33 | `tree_target_small_dataset_exponent_shift` | `tree_target_small_dataset_exponent_shift_1` | `src/writer.rs:1408` | `small-dataset-exponent-shift` | 1 | [`5b6f5b8`](https://github.com/meilisearch/arroy/commit/5b6f5b8de08e87680ad0ac0049602d43717b5a93) |
| 34 | `euclidean_norm_no_sqrt` | `euclidean_norm_no_sqrt_1` | `src/distance/euclidean.rs:56` | `missing-norm-square-root` | 10 | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| 35 | `two_means_weighted_distance_left` | `two_means_weighted_distance_left_1` | `src/distance/mod.rs:175` | `missing-centroid-weighting` | 9 | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| 36 | `euclidean_two_means_cosine_flag` | `euclidean_two_means_cosine_flag_1` | `src/distance/euclidean.rs:71` | `wrong-cosine-flag-in-euclidean-split` | 11 | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| 37 | `insert_descendants_right_guard` | `insert_descendants_right_guard_1` | `src/writer.rs:1516` | `right-descendant-recursion-guard-inverted` | 5 | [`af2cdca`](https://github.com/meilisearch/arroy/commit/af2cdcae43ab5ca6b77ad6cad50df5e64af43b82) |
| 38 | `reader_output_skip_normalization` | `reader_output_skip_normalization_1` | `src/reader.rs:460` | `missing-distance-normalization` | 4 | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| 39 | `euclidean_built_distance_dot` | `euclidean_built_distance_dot_1` | `src/distance/euclidean.rs:46` | `dot-product-used-as-distance` | 15 | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| 40 | `insert_descendants_merge_operator` | `insert_descendants_merge_operator_1` | `src/writer.rs:1462` | `descendant-merge-intersection` | 6 | [`af2cdca`](https://github.com/meilisearch/arroy/commit/af2cdcae43ab5ca6b77ad6cad50df5e64af43b82) |
| 41 | `insert_descendants_left_guard` | `insert_descendants_left_guard_1` | `src/writer.rs:1490` | `left-descendant-recursion-guard-inverted` | 3 | [`af2cdca`](https://github.com/meilisearch/arroy/commit/af2cdcae43ab5ca6b77ad6cad50df5e64af43b82) |
| 42 | `distance_normalized_distance_no_sqrt` | `distance_normalized_distance_no_sqrt_1` | `src/distance/mod.rs:60` | `missing-distance-root-normalization` | 4 | [`d5a7e6f`](https://github.com/meilisearch/arroy/commit/d5a7e6f6f5d8204d60ec602e5f1dbccc238a1601) |
| 43 | `distance_pq_left_margin_sign` | `distance_pq_left_margin_sign_1` | `src/distance/mod.rs:70` | `wrong-left-margin-sign` | 1 | [`d5a7e6f`](https://github.com/meilisearch/arroy/commit/d5a7e6f6f5d8204d60ec602e5f1dbccc238a1601) |
| 44 | `distance_update_mean_divisor` | `distance_update_mean_divisor_1` | `src/distance/mod.rs:101` | `wrong-centroid-divisor` | 12 | [`d5a7e6f`](https://github.com/meilisearch/arroy/commit/d5a7e6f6f5d8204d60ec602e5f1dbccc238a1601) |
| 45 | `distance_side_sign_flip` | `distance_side_sign_flip_1` | `src/distance/mod.rs:118` | `inverted-side-selection-sign` | 12 | [`a70b1dd`](https://github.com/meilisearch/arroy/commit/a70b1dd3c46e75901d9de86070f3167aad6ea2be) |

## Bug Details

### Bug 1: `tmpfile_missing_seek`

| | |
|---|---|
| **File** | `src/parallel.rs:41` |
| **Variant** | `tmpfile_missing_seek_1` |
| **Tags** | `parallel, io, append, issue-143` |
| **Fix commit** | [`a63f097`](https://github.com/meilisearch/arroy/commit/a63f0979b216dde10d50fdfa4fadcb2b1dea73c7) |
| **Fix context** | fix #143 (2025-06-19) |
| **Related links** | [PR #144](https://github.com/meilisearch/arroy/pull/144), [Issue #143](https://github.com/meilisearch/arroy/issues/143) |
| **Type** | `missing-seek` |
| **Test mode** | `debug` |
| **Failure profile** | Heed decode error after read to append overwrite |

**Detecting tests**

- `tests::tmp_nodes::test_read_then_append_keeps_previous_nodes_intact`

### Bug 2: `reader_empty_search_guard`

| | |
|---|---|
| **File** | `src/reader.rs:323` |
| **Variant** | `reader_empty_search_guard_1` |
| **Tags** | `reader, search, empty-index, issue-74` |
| **Fix commit** | [`2128749`](https://github.com/meilisearch/arroy/commit/2128749806c67ca8853395e34bf0b4df1f9e41bc) |
| **Fix context** | fix a panic at search time when the index is empty (2024-06-19) |
| **Related links** | [PR #76](https://github.com/meilisearch/arroy/pull/76), [Issue #74](https://github.com/meilisearch/arroy/issues/74) |
| **Type** | `missing-base-case` |
| **Test mode** | `debug` |
| **Failure profile** | panic from integer logarithm on empty items |

**Detecting tests**

- `tests::reader::search_in_empty_database`
- `tests::writer::delete_document_in_an_empty_index_74`

### Bug 3: `tree_target_rounding`

| | |
|---|---|
| **File** | `src/writer.rs:1423` |
| **Variant** | `tree_target_rounding_1` |
| **Tags** | `writer, tree-count, rounding` |
| **Fix commit** | [`5b6f5b8`](https://github.com/meilisearch/arroy/commit/5b6f5b8de08e87680ad0ac0049602d43717b5a93) |
| **Fix context** | fix the number of tree nodes required per items (2024-01-03) |
| **Related links** | [PR #49](https://github.com/meilisearch/arroy/pull/49) |
| **Type** | `wrong-rounding` |
| **Test mode** | `debug` |
| **Failure profile** | snapshot regressions in tree count behavior |

**Detecting tests**

- `tests::writer::delete_extraneous_tree`
- `tests::writer::guess_right_number_of_tree_while_growing`

### Bug 4: `fit_in_memory_remove_by_rank`

| | |
|---|---|
| **File** | `src/writer.rs:1672` |
| **Variant** | `fit_in_memory_remove_by_rank_1` |
| **Tags** | `writer, sampling, wrong-remove` |
| **Fix commit** | [`5262b08`](https://github.com/meilisearch/arroy/commit/5262b0866ed6a0b8fe529b170a1700fe4a47ebbd) |
| **Fix context** | fix multiple bugs (2025-06-17) |
| **Related links** | [PR #130](https://github.com/meilisearch/arroy/pull/130) |
| **Type** | `wrong-remove-operation` |
| **Test mode** | `debug` |
| **Failure profile** | fit in memory selection corruption |

**Detecting tests**

- `tests::fit_in_memory::test_random_selection`
- `tests::fit_in_memory::test_partial_fit`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`

### Bug 5: `low_memory_none_handling`

| | |
|---|---|
| **File** | `src/writer.rs:1560` |
| **Variant** | `low_memory_none_handling_1` |
| **Tags** | `writer, low-memory, none-handling` |
| **Fix commit** | [`af2cdca`](https://github.com/meilisearch/arroy/commit/af2cdcae43ab5ca6b77ad6cad50df5e64af43b82) |
| **Fix context** | fix panic when we have low memory (2025-06-17) |
| **Related links** | [PR #130](https://github.com/meilisearch/arroy/pull/130) |
| **Type** | `missing-none-handling` |
| **Test mode** | `debug` |
| **Failure profile** | panic from unwrap on missing tmp node |

**Detecting tests**

- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`

### Bug 6: `upgrade_wrong_metadata_key`

| | |
|---|---|
| **File** | `src/upgrade.rs:111` |
| **Variant** | `upgrade_wrong_metadata_key_1` |
| **Tags** | `upgrade, metadata, field-selection` |
| **Fix commit** | [`f3db126`](https://github.com/meilisearch/arroy/commit/f3db126a274264e90bb0eae70892da8293d74425) |
| **Fix context** | fix a bug around multi-index metadata (2024-11-04) |
| **Related links** | [PR #108](https://github.com/meilisearch/arroy/pull/108) |
| **Type** | `wrong-key-field` |
| **Test mode** | `debug` |
| **Failure profile** | metadata decode mismatch in v0.4 to v0.5 upgrade path |

**Detecting tests**

- `tests::upgrade::upgrade_v0_4_to_v0_5_handles_metadata_per_index`

### Bug 7: `upgrade_splitnode_rewrite_gate`

| | |
|---|---|
| **File** | `src/upgrade.rs:287` |
| **Variant** | `upgrade_splitnode_rewrite_gate_1` |
| **Tags** | `upgrade, splitnode, rewrite-gate` |
| **Fix commit** | [`7639506`](https://github.com/meilisearch/arroy/commit/7639506680d32853e1dc825a1d99ea974a4b1bb7) |
| **Fix context** | Fix the broken update and test (2025-05-22) |
| **Related links** | [PR #127](https://github.com/meilisearch/arroy/pull/127) |
| **Type** | `conditional-rewrite` |
| **Test mode** | `debug` |
| **Failure profile** | split nodes not fully rewritten during upgrade |

**Detecting tests**

- `tests::upgrade::simple_upgrade_v0_6_to_v0_7`
- `tests::upgrade::large_upgrade_v0_6_to_v0_7`

### Bug 8: `tmp_lookup_search_window`

| | |
|---|---|
| **File** | `src/parallel.rs:161` |
| **Variant** | `tmp_lookup_search_window_1` |
| **Tags** | `parallel, tmp, search-window` |
| **Fix commit** | [`c0e66c6`](https://github.com/meilisearch/arroy/commit/c0e66c674c210ad8422f4f3bf6e043a729f5d914) |
| **Fix context** | stop panicking if we don't find the node to remove in the two last elements (2024-01-15) |
| **Related links** | [PR #57](https://github.com/meilisearch/arroy/pull/57) |
| **Type** | `narrow-search-window` |
| **Test mode** | `debug` |
| **Failure profile** | tmp node lookup misses nodes outside recent tail |

**Detecting tests**

- `tests::tmp_nodes::test_put_and_get_tmp_nodes`
- `tests::tmp_nodes::test_read_then_append_keeps_previous_nodes_intact`

### Bug 9: `fit_in_memory_min_split_threshold`

| | |
|---|---|
| **File** | `src/writer.rs:1655` |
| **Variant** | `fit_in_memory_min_split_threshold_1` |
| **Tags** | `writer, fit-memory, split-threshold` |
| **Fix commit** | [`fa62650`](https://github.com/meilisearch/arroy/commit/fa626508854b33d0e6994ba601bf27d9c1315eea) |
| **Fix context** | fix a new bug and add unit tests on fit_in_memory (2025-06-17) |
| **Related links** | [PR #130](https://github.com/meilisearch/arroy/pull/130) |
| **Type** | `off-by-one-split-threshold` |
| **Test mode** | `debug` |
| **Failure profile** | timeout/non-termination in low-memory writer path |

**Detecting tests**

- _No explicit failing test captured (timeout or harness-level failure)._

### Bug 10: `missing_tree_count_direction`

| | |
|---|---|
| **File** | `src/writer.rs:551` |
| **Variant** | `missing_tree_count_direction_1` |
| **Tags** | `writer, tree-count, missing-tree` |
| **Fix commit** | [`eaeb864`](https://github.com/meilisearch/arroy/commit/eaeb864f8f9250893fd6fd110729d7dabae38c03) |
| **Fix context** | review and update all the writer + reader tests snapshots + fix a small bug (2025-03-31) |
| **Related links** | [PR #116](https://github.com/meilisearch/arroy/pull/116) |
| **Type** | `wrong-missing-tree-count` |
| **Test mode** | `debug` |
| **Failure profile** | target tree backfill skipped due inverted saturating subtraction |

**Detecting tests**

- `tests::reader::filtering`
- `tests::reader::two_dimension_on_a_column`
- `tests::reader::two_dimension_on_a_line`
- `tests::writer::add_one_item_incrementally`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::delete_one_item`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::reuse_node_id`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::write_and_update_lot_of_random_points`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`
- `tests::writer::write_vectors_until_there_is_a_split`

### Bug 11: `extraneous_tree_count_direction`

| | |
|---|---|
| **File** | `src/writer.rs:652` |
| **Variant** | `extraneous_tree_count_direction_1` |
| **Tags** | `writer, tree-count, extraneous-tree` |
| **Fix commit** | [`66068b4`](https://github.com/meilisearch/arroy/commit/66068b485aad6bd7c5a720325b9237c32a79c132) |
| **Fix context** | fix, improve and test the targetted number of tree (2025-04-02) |
| **Related links** | [PR #116](https://github.com/meilisearch/arroy/pull/116) |
| **Type** | `wrong-extraneous-tree-count` |
| **Test mode** | `debug` |
| **Failure profile** | extraneous tree pruning count inverted; tree layout diverges |

**Detecting tests**

- `tests::writer::delete_extraneous_tree`
- `tests::writer::reuse_node_id`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`

### Bug 12: `delete_empty_left_child_promote`

| | |
|---|---|
| **File** | `src/writer.rs:1083` |
| **Variant** | `delete_empty_left_child_promote_1` |
| **Tags** | `writer, delete, empty-child` |
| **Fix commit** | [`c6c53c5`](https://github.com/meilisearch/arroy/commit/c6c53c5630e5db417492147e4533908abd05479a) |
| **Fix context** | fix #117 (2025-04-01) |
| **Related links** | [PR #116](https://github.com/meilisearch/arroy/pull/116), [Issue #117](https://github.com/meilisearch/arroy/issues/117) |
| **Type** | `stale-empty-child-promotion` |
| **Test mode** | `debug` |
| **Failure profile** | duplicate tmp-node deletion panic in low-memory update path |

**Detecting tests**

- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`

### Bug 13: `fit_descendant_strict_threshold`

| | |
|---|---|
| **File** | `src/writer.rs:476` |
| **Variant** | `fit_descendant_strict_threshold_1` |
| **Tags** | `writer, descendant, threshold` |
| **Fix commit** | [`dfc3e60`](https://github.com/meilisearch/arroy/commit/dfc3e600d35bacb937e3d4e170525ab03d74ecb6) |
| **Fix context** | fix all the edges cases when all the elements fits inside a single descendant (2024-01-09) |
| **Related links** | [PR #41](https://github.com/meilisearch/arroy/pull/41) |
| **Type** | `strict-fit-threshold` |
| **Test mode** | `debug` |
| **Failure profile** | strict descendant threshold causes early split-node expansion |

**Detecting tests**

- `tests::reader::two_dimension_on_a_line`
- `tests::writer::add_one_item_incrementally`
- `tests::writer::add_one_item_incrementally_in_a_one_item_db`
- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::append`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::delete_one_item_in_a_descendant`
- `tests::writer::delete_one_item`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::reuse_node_id`
- `tests::writer::write_vectors_until_there_is_a_descendants`
- `tests::writer::write_and_update_lot_of_random_points`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`

### Bug 14: `single_leaf_root_id_mismatch`

| | |
|---|---|
| **File** | `src/writer.rs:957` |
| **Variant** | `single_leaf_root_id_mismatch_1` |
| **Tags** | `writer, single-leaf, root-id` |
| **Fix commit** | [`dfc3e60`](https://github.com/meilisearch/arroy/commit/dfc3e600d35bacb937e3d4e170525ab03d74ecb6) |
| **Fix context** | fix all the edges cases when all the elements fits inside a single descendant (2024-01-09) |
| **Related links** | [PR #41](https://github.com/meilisearch/arroy/pull/41) |
| **Type** | `wrong-single-leaf-root-id` |
| **Test mode** | `debug` |
| **Failure profile** | single-leaf metadata points to missing tree id |

**Detecting tests**

- `tests::reader::search_in_db_with_a_single_vector`
- `tests::binary_quantized::write_and_retrieve_binary_quantized_vector`
- `tests::writer::add_one_item_incrementally_in_a_one_item_db`
- `tests::writer::add_one_item_incrementally_in_an_empty_db`
- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::append`
- `tests::writer::delete_document_in_an_empty_index_74`
- `tests::writer::delete_one_item_in_a_descendant`
- `tests::writer::delete_one_item_in_a_one_item_db`
- `tests::writer::delete_one_item_in_a_single_document_database`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::use_u32_max_minus_one_for_a_vec`
- `tests::writer::use_u32_max_for_a_vec`
- `tests::writer::write_multiple_indexes`
- `tests::writer::write_one_vector`
- `tests::writer::write_one_vector_in_multiple_trees`
- `tests::writer::write_one_vector_in_one_tree`
- `tests::writer::write_vectors_until_there_is_a_descendants`

### Bug 15: `tree_target_small_dataset_boundary`

| | |
|---|---|
| **File** | `src/writer.rs:1415` |
| **Variant** | `tree_target_small_dataset_boundary_1` |
| **Tags** | `writer, tree-count, threshold` |
| **Fix commit** | [`92c1ddd`](https://github.com/meilisearch/arroy/commit/92c1dddd2d59f6d7a7a4069c7f746d963d9dd08a) |
| **Fix context** | final version of the nb tree guesser (2025-07-07) |
| **Related links** | [PR #138](https://github.com/meilisearch/arroy/pull/138) |
| **Type** | `small-dataset-threshold` |
| **Test mode** | `debug` |
| **Failure profile** | small-dataset threshold includes 10k boundary and shifts tree target snapshots |

**Detecting tests**

- `tests::writer::guess_right_number_of_tree_while_growing`

### Bug 16: `tree_target_dimension_power`

| | |
|---|---|
| **File** | `src/writer.rs:1401` |
| **Variant** | `tree_target_dimension_power_1` |
| **Tags** | `writer, tree-count, dim-scaling` |
| **Fix commit** | [`92c1ddd`](https://github.com/meilisearch/arroy/commit/92c1dddd2d59f6d7a7a4069c7f746d963d9dd08a) |
| **Fix context** | final version of the nb tree guesser (2025-07-07) |
| **Related links** | [PR #138](https://github.com/meilisearch/arroy/pull/138) |
| **Type** | `dimension-scaling-power` |
| **Test mode** | `debug` |
| **Failure profile** | dimension scaling exponent changed in tree target formula |

**Detecting tests**

- `tests::writer::guess_right_number_of_tree_while_growing`

### Bug 17: `euclidean_split_skip_normalize`

| | |
|---|---|
| **File** | `src/distance/euclidean.rs:84` |
| **Variant** | `euclidean_split_skip_normalize_1` |
| **Tags** | `distance, euclidean, split-normalization` |
| **Fix commit** | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| **Fix context** | fix the euclidean distance (2024-09-09) |
| **Related links** | [PR #82](https://github.com/meilisearch/arroy/pull/82) |
| **Type** | `missing-split-normalization` |
| **Test mode** | `debug` |
| **Failure profile** | euclidean split normal is left unnormalized |

**Detecting tests**

- `tests::writer::add_one_item_incrementally`
- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::delete_one_item`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_and_update_lot_of_random_points`
- `tests::writer::write_vectors_until_there_is_a_split`

### Bug 18: `median_top_k_truncate_bias`

| | |
|---|---|
| **File** | `src/reader.rs:707` |
| **Variant** | `median_top_k_truncate_bias_1` |
| **Tags** | `reader, top-k, threshold` |
| **Fix commit** | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| **Fix context** | apply review suggestions, simplify top k fn (2025-05-26) |
| **Related links** | [PR #129](https://github.com/meilisearch/arroy/pull/129) |
| **Type** | `early-buffer-truncation` |
| **Test mode** | `debug` |
| **Failure profile** | median top-k truncates too aggressively during thresholding |

**Detecting tests**

- `tests::reader::two_dimension_on_a_line`
- `tests::reader::median_top_k_vs_binary_heap`

### Bug 19: `euclidean_split_bias_scale`

| | |
|---|---|
| **File** | `src/distance/euclidean.rs:99` |
| **Variant** | `euclidean_split_bias_scale_1` |
| **Tags** | `distance, euclidean, split-bias` |
| **Fix commit** | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| **Fix context** | fix the euclidean distance (2024-09-09) |
| **Related links** | [PR #82](https://github.com/meilisearch/arroy/pull/82) |
| **Type** | `wrong-split-bias-scale` |
| **Test mode** | `debug` |
| **Failure profile** | euclidean split bias is over-scaled |

**Detecting tests**

- `tests::writer::add_one_item_incrementally`
- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::delete_one_item`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_vectors_until_there_is_a_split`
- `tests::writer::write_and_update_lot_of_random_points`

### Bug 20: `median_top_k_final_truncate`

| | |
|---|---|
| **File** | `src/reader.rs:732` |
| **Variant** | `median_top_k_final_truncate_1` |
| **Tags** | `reader, top-k, off-by-one` |
| **Fix commit** | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| **Fix context** | apply review suggestions, simplify top k fn (2025-05-26) |
| **Related links** | [PR #129](https://github.com/meilisearch/arroy/pull/129) |
| **Type** | `off-by-one-final-truncate` |
| **Test mode** | `debug` |
| **Failure profile** | final top-k truncation drops one nearest neighbor |

**Detecting tests**

- `tests::reader::search_in_db_with_a_single_vector`
- `tests::upgrade::simple_upgrade_v0_6_to_v0_7`
- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::reader::filtering`
- `tests::reader::median_top_k_vs_binary_heap`
- `tests::reader::two_dimension_on_a_column`
- `tests::reader::two_dimension_on_a_line`

### Bug 21: `euclidean_margin_sign`

| | |
|---|---|
| **File** | `src/distance/euclidean.rs:111` |
| **Variant** | `euclidean_margin_sign_1` |
| **Tags** | `distance, euclidean, margin-sign` |
| **Fix commit** | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| **Fix context** | fix the euclidean distance (2024-09-09) |
| **Related links** | [PR #82](https://github.com/meilisearch/arroy/pull/82) |
| **Type** | `inverted-margin-sign` |
| **Test mode** | `debug` |
| **Failure profile** | euclidean split margin uses inverted sign |

**Detecting tests**

- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::add_one_item_incrementally`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::delete_one_item`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::reader::two_dimension_on_a_line`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_vectors_until_there_is_a_split`
- `tests::writer::write_and_update_lot_of_random_points`

### Bug 22: `median_top_k_filter_inversion`

| | |
|---|---|
| **File** | `src/reader.rs:693` |
| **Variant** | `median_top_k_filter_inversion_1` |
| **Tags** | `reader, top-k, filter` |
| **Fix commit** | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| **Fix context** | apply review suggestions, simplify top k fn (2025-05-26) |
| **Related links** | [PR #129](https://github.com/meilisearch/arroy/pull/129) |
| **Type** | `inverted-threshold-filter` |
| **Test mode** | `debug` |
| **Failure profile** | top-k candidate filter direction inverted |

**Detecting tests**

- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::reader::median_top_k_vs_binary_heap`

### Bug 23: `median_top_k_sort_direction`

| | |
|---|---|
| **File** | `src/reader.rs:725` |
| **Variant** | `median_top_k_sort_direction_1` |
| **Tags** | `reader, top-k, ordering` |
| **Fix commit** | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| **Fix context** | apply review suggestions, simplify top k fn (2025-05-26) |
| **Related links** | [PR #129](https://github.com/meilisearch/arroy/pull/129) |
| **Type** | `reversed-ordering` |
| **Test mode** | `debug` |
| **Failure profile** | top-k ordering reversed before truncation |

**Detecting tests**

- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::upgrade::simple_upgrade_v0_6_to_v0_7`
- `tests::reader::filtering`
- `tests::reader::median_top_k_vs_binary_heap`
- `tests::reader::two_dimension_on_a_line`
- `tests::reader::two_dimension_on_a_column`

### Bug 24: `reader_searchk_root_factor`

| | |
|---|---|
| **File** | `src/reader.rs:335` |
| **Variant** | `reader_searchk_root_factor_1` |
| **Tags** | `reader, search, budget` |
| **Fix commit** | [`3538b81`](https://github.com/meilisearch/arroy/commit/3538b817105bac074f601008fbaba594eb5d121a) |
| **Fix context** | Implement the search nns API (2023-11-16) |
| **Related links** | [PR #1](https://github.com/meilisearch/arroy/pull/1) |
| **Type** | `missing-root-factor` |
| **Test mode** | `debug` |
| **Failure profile** | search budget ignores root-count factor |

**Detecting tests**

- `tests::reader::filtering`
- `tests::reader::two_dimension_on_a_column`
- `tests::reader::two_dimension_on_a_line`

### Bug 25: `reader_candidates_filter_bypass`

| | |
|---|---|
| **File** | `src/reader.rs:374` |
| **Variant** | `reader_candidates_filter_bypass_1` |
| **Tags** | `reader, search, candidates` |
| **Fix commit** | [`3e7d3d0`](https://github.com/meilisearch/arroy/commit/3e7d3d0926e26d60e16f0f10ddf82596f782670b) |
| **Fix context** | add a test of the filtering (2023-12-05) |
| **Related links** | [PR #38](https://github.com/meilisearch/arroy/pull/38) |
| **Type** | `candidate-filter-bypass` |
| **Test mode** | `debug` |
| **Failure profile** | candidate-filter bypass in descendants expansion |

**Detecting tests**

- `tests::reader::filtering`

### Bug 26: `reader_margin_argument_order`

| | |
|---|---|
| **File** | `src/reader.rs:399` |
| **Variant** | `reader_margin_argument_order_1` |
| **Tags** | `reader, search, margin` |
| **Fix commit** | [`3538b81`](https://github.com/meilisearch/arroy/commit/3538b817105bac074f601008fbaba594eb5d121a) |
| **Fix context** | Implement the search nns API (2023-11-16) |
| **Related links** | [PR #1](https://github.com/meilisearch/arroy/pull/1) |
| **Type** | `swapped-margin-arguments` |
| **Test mode** | `debug` |
| **Failure profile** | margin argument order swapped |

**Detecting tests**

- `tests::reader::two_dimension_on_a_line`

### Bug 27: `reader_split_side_distance_swap`

| | |
|---|---|
| **File** | `src/reader.rs:412` |
| **Variant** | `reader_split_side_distance_swap_1` |
| **Tags** | `reader, search, branch-priority` |
| **Fix commit** | [`3538b81`](https://github.com/meilisearch/arroy/commit/3538b817105bac074f601008fbaba594eb5d121a) |
| **Fix context** | Implement the search nns API (2023-11-16) |
| **Related links** | [PR #1](https://github.com/meilisearch/arroy/pull/1) |
| **Type** | `swapped-branch-priority` |
| **Test mode** | `debug` |
| **Failure profile** | left/right branch priority distance swapped |

**Detecting tests**

- `tests::reader::two_dimension_on_a_line`

### Bug 28: `reader_nns_dedup_order`

| | |
|---|---|
| **File** | `src/reader.rs:427` |
| **Variant** | `reader_nns_dedup_order_1` |
| **Tags** | `reader, search, dedup` |
| **Fix commit** | [`3538b81`](https://github.com/meilisearch/arroy/commit/3538b817105bac074f601008fbaba594eb5d121a) |
| **Fix context** | Implement the search nns API (2023-11-16) |
| **Related links** | [PR #1](https://github.com/meilisearch/arroy/pull/1) |
| **Type** | `dedup-before-sort` |
| **Test mode** | `debug` |
| **Failure profile** | dedup performed before sorting nearest-neighbor IDs |

**Detecting tests**

- `tests::reader::filtering`
- `tests::reader::two_dimension_on_a_column`
- `tests::reader::two_dimension_on_a_line`

### Bug 29: `median_top_k_initial_threshold`

| | |
|---|---|
| **File** | `src/reader.rs:679` |
| **Variant** | `median_top_k_initial_threshold_1` |
| **Tags** | `reader, top-k, threshold-init` |
| **Fix commit** | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| **Fix context** | apply review suggestions, simplify top k fn (2025-05-26) |
| **Related links** | [PR #129](https://github.com/meilisearch/arroy/pull/129) |
| **Type** | `wrong-threshold-initialization` |
| **Test mode** | `debug` |
| **Failure profile** | top-k threshold initialized to minimum value |

**Detecting tests**

- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::reader::median_top_k_vs_binary_heap`

### Bug 30: `reader_leaf_candidates_default_false`

| | |
|---|---|
| **File** | `src/reader.rs:361` |
| **Variant** | `reader_leaf_candidates_default_false_1` |
| **Tags** | `reader, search, candidates` |
| **Fix commit** | [`3e7d3d0`](https://github.com/meilisearch/arroy/commit/3e7d3d0926e26d60e16f0f10ddf82596f782670b) |
| **Fix context** | add a test of the filtering (2023-12-05) |
| **Related links** | [PR #38](https://github.com/meilisearch/arroy/pull/38) |
| **Type** | `wrong-candidate-default` |
| **Test mode** | `debug` |
| **Failure profile** | leaf candidate filter defaults to excluding all items when no candidate set is provided |

**Detecting tests**

- `tests::upgrade::simple_upgrade_v0_6_to_v0_7`

### Bug 31: `reader_descendant_no_candidates_skip`

| | |
|---|---|
| **File** | `src/reader.rs:382` |
| **Variant** | `reader_descendant_no_candidates_skip_1` |
| **Tags** | `reader, search, descendants` |
| **Fix commit** | [`3e7d3d0`](https://github.com/meilisearch/arroy/commit/3e7d3d0926e26d60e16f0f10ddf82596f782670b) |
| **Fix context** | add a test of the filtering (2023-12-05) |
| **Related links** | [PR #38](https://github.com/meilisearch/arroy/pull/38) |
| **Type** | `skip-descendants-without-candidates` |
| **Test mode** | `debug` |
| **Failure profile** | descendant expansion is skipped when no candidate filter is provided |

**Detecting tests**

- `tests::reader::search_in_db_with_a_single_vector`
- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::upgrade::simple_upgrade_v0_6_to_v0_7`
- `tests::reader::two_dimension_on_a_column`
- `tests::reader::two_dimension_on_a_line`

### Bug 32: `reader_k_selection_max`

| | |
|---|---|
| **File** | `src/reader.rs:450` |
| **Variant** | `reader_k_selection_max_1` |
| **Tags** | `reader, search, top-k` |
| **Fix commit** | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| **Fix context** | apply review suggestions, simplify top k fn (2025-05-26) |
| **Related links** | [PR #129](https://github.com/meilisearch/arroy/pull/129) |
| **Type** | `max-instead-of-min-top-k` |
| **Test mode** | `debug` |
| **Failure profile** | top-k bound uses max instead of min and over-returns neighbors |

**Detecting tests**

- `tests::upgrade::simple_upgrade_v0_6_to_v0_7`
- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::reader::two_dimension_on_a_column`
- `tests::reader::two_dimension_on_a_line`

### Bug 33: `tree_target_small_dataset_exponent_shift`

| | |
|---|---|
| **File** | `src/writer.rs:1408` |
| **Variant** | `tree_target_small_dataset_exponent_shift_1` |
| **Tags** | `writer, tree-count, small-dataset` |
| **Fix commit** | [`5b6f5b8`](https://github.com/meilisearch/arroy/commit/5b6f5b8de08e87680ad0ac0049602d43717b5a93) |
| **Fix context** | fix the number of tree nodes required per items (2024-01-03) |
| **Related links** | [PR #49](https://github.com/meilisearch/arroy/pull/49) |
| **Type** | `small-dataset-exponent-shift` |
| **Test mode** | `debug` |
| **Failure profile** | small-dataset tree exponent is shifted by +1 |

**Detecting tests**

- `tests::writer::guess_right_number_of_tree_while_growing`

### Bug 34: `euclidean_norm_no_sqrt`

| | |
|---|---|
| **File** | `src/distance/euclidean.rs:56` |
| **Variant** | `euclidean_norm_no_sqrt_1` |
| **Tags** | `distance, euclidean, norm` |
| **Fix commit** | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| **Fix context** | fix the euclidean distance (2024-09-09) |
| **Related links** | [PR #82](https://github.com/meilisearch/arroy/pull/82) |
| **Type** | `missing-norm-square-root` |
| **Test mode** | `debug` |
| **Failure profile** | euclidean norm omits the square root during split normalization |

**Detecting tests**

- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::add_one_item_incrementally`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::delete_one_item`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_and_update_lot_of_random_points`
- `tests::writer::write_vectors_until_there_is_a_split`

### Bug 35: `two_means_weighted_distance_left`

| | |
|---|---|
| **File** | `src/distance/mod.rs:175` |
| **Variant** | `two_means_weighted_distance_left_1` |
| **Tags** | `distance, split, weighting` |
| **Fix commit** | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| **Fix context** | fix the euclidean distance (2024-09-09) |
| **Related links** | [PR #82](https://github.com/meilisearch/arroy/pull/82) |
| **Type** | `missing-centroid-weighting` |
| **Test mode** | `debug` |
| **Failure profile** | left centroid weighting factor is dropped during split clustering |

**Detecting tests**

- `tests::writer::add_one_item_incrementally`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::delete_one_item`
- `tests::writer::reuse_node_id`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::write_vectors_until_there_is_a_split`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`
- `tests::writer::write_and_update_lot_of_random_points`

### Bug 36: `euclidean_two_means_cosine_flag`

| | |
|---|---|
| **File** | `src/distance/euclidean.rs:71` |
| **Variant** | `euclidean_two_means_cosine_flag_1` |
| **Tags** | `distance, euclidean, split-centroid` |
| **Fix commit** | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| **Fix context** | fix the euclidean distance (2024-09-09) |
| **Related links** | [PR #82](https://github.com/meilisearch/arroy/pull/82) |
| **Type** | `wrong-cosine-flag-in-euclidean-split` |
| **Test mode** | `debug` |
| **Failure profile** | euclidean split incorrectly enables cosine normalization in two-means |

**Detecting tests**

- `tests::writer::add_one_item_incrementally`
- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::reader::two_dimension_on_a_line`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::delete_one_item`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_vectors_until_there_is_a_split`
- `tests::writer::write_and_update_lot_of_random_points`

### Bug 37: `insert_descendants_right_guard`

| | |
|---|---|
| **File** | `src/writer.rs:1516` |
| **Variant** | `insert_descendants_right_guard_1` |
| **Tags** | `writer, descendants, right-branch` |
| **Fix commit** | [`af2cdca`](https://github.com/meilisearch/arroy/commit/af2cdcae43ab5ca6b77ad6cad50df5e64af43b82) |
| **Fix context** | fix panic when we have low memory (2025-06-17) |
| **Related links** | [PR #130](https://github.com/meilisearch/arroy/pull/130) |
| **Type** | `right-descendant-recursion-guard-inverted` |
| **Test mode** | `debug` |
| **Failure profile** | right descendants are skipped when right branch is non-empty |

**Detecting tests**

- `tests::writer::add_one_item_incrementally`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_and_update_lot_of_random_points`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`

### Bug 38: `reader_output_skip_normalization`

| | |
|---|---|
| **File** | `src/reader.rs:460` |
| **Variant** | `reader_output_skip_normalization_1` |
| **Tags** | `reader, search, output-distance` |
| **Fix commit** | [`97ba58d`](https://github.com/meilisearch/arroy/commit/97ba58d2e298567fb565019b9df0f75eacc5a1f2) |
| **Fix context** | apply review suggestions, simplify top k fn (2025-05-26) |
| **Related links** | [PR #129](https://github.com/meilisearch/arroy/pull/129) |
| **Type** | `missing-distance-normalization` |
| **Test mode** | `debug` |
| **Failure profile** | reader returns non-normalized distances in search output |

**Detecting tests**

- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::reader::filtering`
- `tests::reader::two_dimension_on_a_column`
- `tests::reader::two_dimension_on_a_line`

### Bug 39: `euclidean_built_distance_dot`

| | |
|---|---|
| **File** | `src/distance/euclidean.rs:46` |
| **Variant** | `euclidean_built_distance_dot_1` |
| **Tags** | `distance, euclidean, built-distance` |
| **Fix commit** | [`60875eb`](https://github.com/meilisearch/arroy/commit/60875ebc0a110a2a8662c72d49af7b2a22284a36) |
| **Fix context** | fix the euclidean distance (2024-09-09) |
| **Related links** | [PR #82](https://github.com/meilisearch/arroy/pull/82) |
| **Type** | `dot-product-used-as-distance` |
| **Test mode** | `debug` |
| **Failure profile** | euclidean built distance uses dot product instead of euclidean metric |

**Detecting tests**

- `tests::upgrade::simple_upgrade_v0_6_to_v0_7`
- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::reader::filtering`
- `tests::reader::two_dimension_on_a_line`
- `tests::writer::add_one_item_incrementally`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::delete_one_item`
- `tests::writer::delete_extraneous_tree`
- `tests::reader::two_dimension_on_a_column`
- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_vectors_until_there_is_a_split`
- `tests::writer::write_and_update_lot_of_random_points`

### Bug 40: `insert_descendants_merge_operator`

| | |
|---|---|
| **File** | `src/writer.rs:1462` |
| **Variant** | `insert_descendants_merge_operator_1` |
| **Tags** | `writer, descendants, merge` |
| **Fix commit** | [`af2cdca`](https://github.com/meilisearch/arroy/commit/af2cdcae43ab5ca6b77ad6cad50df5e64af43b82) |
| **Fix context** | fix panic when we have low memory (2025-06-17) |
| **Related links** | [PR #130](https://github.com/meilisearch/arroy/pull/130) |
| **Type** | `descendant-merge-intersection` |
| **Test mode** | `debug` |
| **Failure profile** | descendant updates intersect instead of union when merging inserts |

**Detecting tests**

- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::add_one_item_incrementally`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_and_update_lot_of_random_points`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`

### Bug 41: `insert_descendants_left_guard`

| | |
|---|---|
| **File** | `src/writer.rs:1490` |
| **Variant** | `insert_descendants_left_guard_1` |
| **Tags** | `writer, descendants, left-branch` |
| **Fix commit** | [`af2cdca`](https://github.com/meilisearch/arroy/commit/af2cdcae43ab5ca6b77ad6cad50df5e64af43b82) |
| **Fix context** | fix panic when we have low memory (2025-06-17) |
| **Related links** | [PR #130](https://github.com/meilisearch/arroy/pull/130) |
| **Type** | `left-descendant-recursion-guard-inverted` |
| **Test mode** | `debug` |
| **Failure profile** | left descendants are skipped when left branch is non-empty |

**Detecting tests**

- `tests::writer::add_one_item_incrementally`
- `tests::writer::write_and_update_lot_of_random_points`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`

### Bug 42: `distance_normalized_distance_no_sqrt`

| | |
|---|---|
| **File** | `src/distance/mod.rs:60` |
| **Variant** | `distance_normalized_distance_no_sqrt_1` |
| **Tags** | `distance, normalization, scaling` |
| **Fix commit** | [`d5a7e6f`](https://github.com/meilisearch/arroy/commit/d5a7e6f6f5d8204d60ec602e5f1dbccc238a1601) |
| **Fix context** | Introduce the Euclidean distance (2023-11-16) |
| **Related links** | _No issue/PR links detected from tags, commit metadata, or GitHub commit associations._ |
| **Type** | `missing-distance-root-normalization` |
| **Test mode** | `debug` |
| **Failure profile** | test failure |

**Detecting tests**

- `tests::upgrade::large_upgrade_v0_6_to_v0_7`
- `tests::reader::filtering`
- `tests::reader::two_dimension_on_a_column`
- `tests::reader::two_dimension_on_a_line`

### Bug 43: `distance_pq_left_margin_sign`

| | |
|---|---|
| **File** | `src/distance/mod.rs:70` |
| **Variant** | `distance_pq_left_margin_sign_1` |
| **Tags** | `distance, queue-priority, margin` |
| **Fix commit** | [`d5a7e6f`](https://github.com/meilisearch/arroy/commit/d5a7e6f6f5d8204d60ec602e5f1dbccc238a1601) |
| **Fix context** | Introduce the Euclidean distance (2023-11-16) |
| **Related links** | _No issue/PR links detected from tags, commit metadata, or GitHub commit associations._ |
| **Type** | `wrong-left-margin-sign` |
| **Test mode** | `debug` |
| **Failure profile** | test failure |

**Detecting tests**

- `tests::reader::two_dimension_on_a_line`

### Bug 44: `distance_update_mean_divisor`

| | |
|---|---|
| **File** | `src/distance/mod.rs:101` |
| **Variant** | `distance_update_mean_divisor_1` |
| **Tags** | `distance, centroid, averaging` |
| **Fix commit** | [`d5a7e6f`](https://github.com/meilisearch/arroy/commit/d5a7e6f6f5d8204d60ec602e5f1dbccc238a1601) |
| **Fix context** | Introduce the Euclidean distance (2023-11-16) |
| **Related links** | _No issue/PR links detected from tags, commit metadata, or GitHub commit associations._ |
| **Type** | `wrong-centroid-divisor` |
| **Test mode** | `debug` |
| **Failure profile** | test failure |

**Detecting tests**

- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::delete_extraneous_tree`
- `tests::reader::two_dimension_on_a_line`
- `tests::writer::add_one_item_incrementally`
- `tests::writer::delete_one_item`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::reuse_node_id`
- `tests::writer::write_vectors_until_there_is_a_split`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`
- `tests::writer::write_and_update_lot_of_random_points`

### Bug 45: `distance_side_sign_flip`

| | |
|---|---|
| **File** | `src/distance/mod.rs:118` |
| **Variant** | `distance_side_sign_flip_1` |
| **Tags** | `distance, side-selection, sign` |
| **Fix commit** | [`a70b1dd`](https://github.com/meilisearch/arroy/commit/a70b1dd3c46e75901d9de86070f3167aad6ea2be) |
| **Fix context** | feat: make normal a Leaf<'_, Distance> (2025-05-29) |
| **Related links** | [PR #132](https://github.com/meilisearch/arroy/pull/132) |
| **Type** | `inverted-side-selection-sign` |
| **Test mode** | `debug` |
| **Failure profile** | test failure |

**Detecting tests**

- `tests::writer::add_one_item_incrementally_to_create_a_split_node`
- `tests::writer::add_one_item_incrementally`
- `tests::reader::two_dimension_on_a_line`
- `tests::writer::create_root_split_node_with_empty_child`
- `tests::writer::delete_one_item`
- `tests::writer::delete_extraneous_tree`
- `tests::writer::overwrite_one_item_incremental`
- `tests::writer::delete_one_leaf_in_a_split`
- `tests::writer::reuse_node_id`
- `tests::writer::write_vectors_until_there_is_a_split`
- `tests::writer::write_and_update_lot_of_random_points`
- `tests::writer::write_and_update_lot_of_random_points_with_little_memory`
