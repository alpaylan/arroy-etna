use std::borrow::Cow;

use insta::assert_debug_snapshot;
use roaring::RoaringBitmap;

use crate::distance::{Cosine, NodeHeaderCosine};
use crate::internals::UnalignedVector;
use crate::node::{Descendants, Leaf, Node, SplitPlaneNormal};
use crate::parallel::TmpNodes;

#[test]
fn test_put_and_get_tmp_nodes() {
    let mut tmp_nodes = TmpNodes::<Cosine>::new().unwrap();
    for i in 0..10 {
        let node = Node::Descendants(Descendants {
            descendants: Cow::Owned(RoaringBitmap::from_iter(&[i + 0, i + 1, i + 2])),
        });
        tmp_nodes.put(i, &node).unwrap();
    }

    assert_debug_snapshot!(tmp_nodes.get(0).unwrap().unwrap(), @r"
    Descendants(
        Descendants {
            descendants: [
                0,
                1,
                2,
            ],
        },
    )
    ");
    assert_debug_snapshot!(tmp_nodes.get(9).unwrap().unwrap(), @r"
    Descendants(
        Descendants {
            descendants: [
                9,
                10,
                11,
            ],
        },
    )
    ");
    assert_debug_snapshot!(tmp_nodes.get(10).unwrap(), @"None");

    // We start at 11 so there will be a hole at the id 10
    for i in 11..20 {
        let normal =
            if i % 2 == 0 { Some(UnalignedVector::from_vec(vec![i as f32])) } else { None };
        let node = Node::SplitPlaneNormal(SplitPlaneNormal {
            left: i * 2,
            right: i * 2 + 1,
            normal: normal.map(|v| Leaf { header: NodeHeaderCosine { norm: 0. }, vector: v }),
        });
        tmp_nodes.put(i, &node).unwrap();
    }

    assert_debug_snapshot!(tmp_nodes.get(10).unwrap(), @"None");
    assert_debug_snapshot!(tmp_nodes.get(11).unwrap().unwrap(), @r#"
    SplitPlaneNormal(
        SplitPlaneNormal<cosine> {
            left: 22,
            right: 23,
            normal: "none",
        },
    )
    "#);

    assert_debug_snapshot!(tmp_nodes.get(15).unwrap().unwrap(), @r#"
    SplitPlaneNormal(
        SplitPlaneNormal<cosine> {
            left: 30,
            right: 31,
            normal: "none",
        },
    )
    "#);

    assert_debug_snapshot!(tmp_nodes.get(19).unwrap().unwrap(), @r#"
    SplitPlaneNormal(
        SplitPlaneNormal<cosine> {
            left: 38,
            right: 39,
            normal: "none",
        },
    )
    "#);

    assert_debug_snapshot!(tmp_nodes.get(20).unwrap(), @"None");

    // can we still get the previous nodes correctly?
    assert_debug_snapshot!(tmp_nodes.get(3).unwrap().unwrap(), @r"
    Descendants(
        Descendants {
            descendants: [
                3,
                4,
                5,
            ],
        },
    )
    ");
}

#[test]
fn test_read_then_append_keeps_previous_nodes_intact() {
    let mut tmp_nodes = TmpNodes::<Cosine>::new().unwrap();

    let mk_desc = |start: u32| {
        Node::Descendants(Descendants {
            descendants: Cow::Owned(RoaringBitmap::from_iter(start..start + 64)),
        })
    };

    // Create enough data to exceed BufReader's internal buffer.
    for i in 0..256u32 {
        tmp_nodes.put(i, &mk_desc(i)).unwrap();
    }

    // Force transition to Reading and leave the cursor near the beginning of the file.
    let _ = tmp_nodes.get(0).unwrap().unwrap();

    // Transition back to Writing and append a new node.
    tmp_nodes.put(10_000, &mk_desc(10_000)).unwrap();

    // All original nodes must remain readable and unchanged.
    for i in 0..256u32 {
        let node = tmp_nodes.get(i).unwrap().unwrap();
        let Node::Descendants(Descendants { descendants }) = node else { unreachable!() };
        assert_eq!(descendants.len(), 64);
        assert!(descendants.contains(i));
        assert!(descendants.contains(i + 63));
    }

    let node = tmp_nodes.get(10_000).unwrap().unwrap();
    let Node::Descendants(Descendants { descendants }) = node else { unreachable!() };
    assert_eq!(descendants.len(), 64);
    assert!(descendants.contains(10_000));
    assert!(descendants.contains(10_063));
}
