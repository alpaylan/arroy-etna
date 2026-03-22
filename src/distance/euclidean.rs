use std::fmt;

use bytemuck::{Pod, Zeroable};
use rand::Rng;

use super::two_means;
use crate::distance::Distance;
use crate::node::Leaf;
use crate::parallel::ImmutableSubsetLeafs;
use crate::spaces::simple::{dot_product, euclidean_distance};
use crate::unaligned_vector::UnalignedVector;

/// The Euclidean distance between two points in Euclidean space
/// is the length of the line segment between them.
///
/// `d(p, q) = sqrt((p - q)²)`
#[derive(Debug, Clone)]
pub enum Euclidean {}

/// The header of Euclidean leaf nodes.
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct NodeHeaderEuclidean {
    /// An extra constant term to determine the offset of the plane
    bias: f32,
}
impl fmt::Debug for NodeHeaderEuclidean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeHeaderEuclidean").field("bias", &format!("{:.4}", self.bias)).finish()
    }
}

impl Distance for Euclidean {
    type Header = NodeHeaderEuclidean;
    type VectorCodec = f32;

    fn name() -> &'static str {
        "euclidean"
    }

    fn new_header(_vector: &UnalignedVector<Self::VectorCodec>) -> Self::Header {
        NodeHeaderEuclidean { bias: 0.0 }
    }

    fn built_distance(p: &Leaf<Self>, q: &Leaf<Self>) -> f32 {
        /*| euclidean_built_distance_dot [distance, euclidean, built-distance] */
        euclidean_distance(&p.vector, &q.vector)
        /*|| euclidean_built_distance_dot_1 */
        /*|
        dot_product(&p.vector, &q.vector)
        */
        /* |*/
    }

    fn norm_no_header(v: &UnalignedVector<Self::VectorCodec>) -> f32 {
        /*| euclidean_norm_no_sqrt [distance, euclidean, norm] */
        dot_product(v, v).sqrt()
        /*|| euclidean_norm_no_sqrt_1 */
        /*|
        dot_product(v, v)
        */
        /* |*/
    }

    fn init(_node: &mut Leaf<Self>) {}

    fn create_split<'a, R: Rng>(
        children: &'a ImmutableSubsetLeafs<Self>,
        rng: &mut R,
    ) -> heed::Result<Leaf<'a, Self>> {
        /*| euclidean_two_means_cosine_flag [distance, euclidean, split-centroid] */
        let [node_p , node_q] = two_means (rng , children , false) ?;
        /*|| euclidean_two_means_cosine_flag_1 */
        /*|
        let [node_p , node_q] = two_means (rng , children , true) ?;
        */
        /* |*/
        let vector: Vec<_> =
            node_p.vector.iter().zip(node_q.vector.iter()).map(|(p, q)| p - q).collect();
        let mut normal: Leaf<'static, Self> = Leaf {
            header: NodeHeaderEuclidean { bias: 0.0 },
            vector: UnalignedVector::from_vec(vector),
        };
        /*| euclidean_split_skip_normalize [distance, euclidean, split-normalization] */
        Self::normalize(&mut normal);
        /*|| euclidean_split_skip_normalize_1 */
        /*|
        ()
        */
        /* |*/

        let bias: f32 = normal
            .vector
            .iter()
            .zip(node_p.vector.iter())
            .zip(node_q.vector.iter())
            .map(|((n, p), q)| -n * (p + q) / 2.0)
            .sum();
        /*| euclidean_split_bias_scale [distance, euclidean, split-bias] */
        normal.header.bias = bias;
        /*|| euclidean_split_bias_scale_1 */
        /*|
        normal.header.bias = bias * 2.0;
        */
        /* |*/

        Ok(normal)
    }

    fn margin(n: &Leaf<Self>, q: &Leaf<Self>) -> f32 {
        /*| euclidean_margin_sign [distance, euclidean, margin-sign] */
        n.header.bias + dot_product(&n.vector, &q.vector)
        /*|| euclidean_margin_sign_1 */
        /*|
        n.header.bias - dot_product(&n.vector, &q.vector)
        */
        /* |*/
    }
}
