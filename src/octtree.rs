use std::ops::RangeInclusive;

use anyhow::{bail, Result};
use itertools::Itertools;

/// An n-dimensional octtree, splitting at a predefined list of points
#[derive(Debug)]
pub struct Octtree<const N: usize> {
    split_points: [Vec<Coordinate>; N],
    nodes: Vec<Node<N>>,
}

pub type Coordinate = i64;

#[derive(Clone, Copy, Debug)]
struct NodeIndex(u32);

impl NodeIndex {
    fn idx(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug)]
enum Node<const N: usize> {
    Leaf {
        filled: bool,
    },
    Interior {
        children: [NodeIndex; 2],
        split_axis: u8,
    },
}

impl<const N: usize> Octtree<N> {
    /// Create a new empty octtree
    ///
    /// The split_points denote the coordinates at which the tree will be
    /// subdivided.
    pub fn new(mut split_points: [Vec<Coordinate>; N]) -> Self {
        for s in &mut split_points {
            s.sort();
            *s = s.drain(..).unique().collect();
        }

        Self {
            split_points,
            nodes: vec![Node::Leaf { filled: false }],
        }
    }

    pub fn nr_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Either completely fill or clear an area. Note that all the boundaries
    /// have to be on the split points provided when constructing the octtree.
    pub fn set_region(
        &mut self,
        bounds: [RangeInclusive<Coordinate>; N],
        filled: bool,
    ) -> Result<()> {
        let bounds_exclusive = bounds.map(|range| (*range.start(), range.end() + 1));

        for (bound, dim) in bounds_exclusive.iter().zip(0..) {
            if self.split_points[dim].binary_search(&bound.0).is_err()
                || self.split_points[dim].binary_search(&bound.1).is_err()
            {
                bail!("bounds in dimension {} are not in split list", dim);
            }
        }

        let split_points = self.split_points.each_ref().map(|sp| sp.as_slice());
        set_inner(
            &mut self.nodes,
            NodeIndex(0),
            split_points,
            bounds_exclusive,
            filled,
        );

        Ok(())
    }

    pub fn nr_filled(&self) -> Coordinate {
        let split_points = self.split_points.each_ref().map(|sp| sp.as_slice());
        count_filled(&self.nodes, NodeIndex(0), split_points)
    }
}

fn count_filled<const N: usize>(
    nodes: &Vec<Node<N>>,
    current_node: NodeIndex,
    mut split_points: [&[Coordinate]; N],
) -> Coordinate {
    match nodes[current_node.idx()] {
        Node::Leaf { filled: true } => split_points
            .iter()
            .map(|sp| sp.last().unwrap() - sp.first().unwrap())
            .product(),
        Node::Leaf { filled: false } => 0,
        Node::Interior {
            children,
            split_axis,
        } => {
            let dim = split_axis as usize;
            let range = split_points[dim];
            let middle_idx = range.len() / 2;

            split_points[dim] = &range[..middle_idx + 1];
            let left = count_filled(nodes, children[0], split_points);
            split_points[dim] = &range[middle_idx..];
            let right = count_filled(nodes, children[1], split_points);
            left + right
        }
    }
}

fn set_inner<const N: usize>(
    nodes: &mut Vec<Node<N>>,
    current_node: NodeIndex,
    mut split_points: [&[Coordinate]; N],
    bounds: [(Coordinate, Coordinate); N],
    fill: bool,
) {
    let mut node = nodes[current_node.idx()];
    if let Node::Leaf { filled } = node {
        if completely_in_bounds(split_points, bounds) {
            // don't need to split further, we're covering the entire node
            node = Node::Leaf { filled: fill };
        } else {
            // split the node into two new sub nodes
            let split_axis = split_points
                .iter()
                .position_max_by_key(|sp| sp.len())
                .unwrap();
            assert!(split_points[split_axis].len() > 2);

            let idx_a = NodeIndex(nodes.len() as u32);
            nodes.push(Node::Leaf { filled });
            let idx_b = NodeIndex(nodes.len() as u32);
            nodes.push(Node::Leaf { filled });

            node = Node::Interior {
                children: [idx_a, idx_b],
                split_axis: split_axis as u8,
            };
        }
    }

    nodes[current_node.idx()] = node;

    if let Node::Interior {
        children,
        split_axis,
    } = node
    {
        // descend into the sub nodes
        let dim = split_axis as usize;
        let range = split_points[dim];
        let middle_idx = range.len() / 2;
        let middle_val = range[middle_idx];

        let bound = bounds[dim];
        if bound.0 < middle_val {
            split_points[dim] = &range[..middle_idx + 1];
            set_inner(nodes, children[0], split_points, bounds, fill);
        }
        if bound.1 > middle_val {
            split_points[dim] = &range[middle_idx..];
            set_inner(nodes, children[1], split_points, bounds, fill);
        }
    }
}

fn completely_in_bounds<const N: usize>(
    split_points: [&[Coordinate]; N],
    bounds: [(Coordinate, Coordinate); N],
) -> bool {
    for (sp, bound) in split_points.iter().zip(bounds.iter()) {
        if !(bound.0 <= *sp.first().unwrap() && bound.1 >= *sp.last().unwrap()) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_has_zero_filled() {
        let tree = Octtree::new([vec![1, 2], vec![1, 2, 3]]);
        assert_eq!(tree.nr_filled(), 0);
    }

    #[test]
    fn test_1d_fill_entire_range() {
        let mut tree = Octtree::new([vec![0, 5]]);
        tree.set_region([0..=4], true).unwrap();
        assert_eq!(tree.nr_filled(), 5);
    }

    #[test]
    fn test_1d_fill_subsegment() {
        let mut tree = Octtree::new([vec![0, 1, 4, 5]]);
        tree.set_region([1..=3], true).unwrap();
        assert_eq!(tree.nr_filled(), 3);
    }

    #[test]
    fn test_1d_overlapping() {
        let mut tree = Octtree::new([vec![0, 1, 2, 3, 4, 5]]);
        tree.set_region([1..=4], true).unwrap();
        tree.set_region([2..=4], true).unwrap();
        assert_eq!(tree.nr_filled(), 4);
    }

    #[test]
    fn test_1d_subtracting() {
        let mut tree = Octtree::new([vec![0, 1, 2, 3, 4, 5]]);
        tree.set_region([1..=4], true).unwrap();
        tree.set_region([2..=3], false).unwrap();
        assert_eq!(tree.nr_filled(), 2);
    }

    #[test]
    fn test_2d() {
        let mut tree = Octtree::new([vec![0, 1, 3, 4], vec![0, 1, 3]]);
        tree.set_region([0..=2, 0..=2], true).unwrap();
        tree.set_region([1..=3, 1..=2], true).unwrap();
        assert_eq!(tree.nr_filled(), 11);
    }
}
