#![allow(dead_code)]
pub mod graph;
pub mod input;

use std::{collections::HashMap, path::Path};

use input::EdgeList;
use thiserror::Error;

#[cfg(feature = "size32")]
pub(crate) type Node = u32;
#[cfg(feature = "size32")]
pub(crate) type AtomicNode = std::sync::atomic::AtomicU32;
#[cfg(feature = "size32")]
#[inline]
pub(crate) fn as_usize(node: Node) -> usize {
    node as usize
}

#[cfg(not(feature = "size32"))]
pub(crate) type Node = usize;
#[cfg(not(feature = "size32"))]
pub(crate) type AtomicNode = std::sync::atomic::AtomicUsize;
#[cfg(not(feature = "size32"))]
#[inline]
pub(crate) fn as_usize(node: Node) -> usize {
    node
}

#[derive(Error, Debug)]
pub enum Error {}

pub trait Graph {
    fn node_count(&self) -> Node;

    fn edge_count(&self) -> Node;
}

pub trait UndirectedGraph: Graph {
    fn degree(&self, node: Node) -> Node;

    fn neighbors(&self, node: Node) -> &[Node];
}

pub trait DirectedGraph: Graph {
    fn out_degree(&self, node: Node) -> Node;

    fn out_neighbors(&self, node: Node) -> &[Node];

    fn in_degree(&self, node: Node) -> Node;

    fn in_neighbors(&self, node: Node) -> &[Node];
}

pub trait NodeLabeledGraph: Graph {
    fn label(&self, node: Node) -> Node;

    fn nodes_by_label(&self, label: Node) -> &[Node];

    fn label_count(&self) -> Node;

    fn max_label(&self) -> Node;

    fn max_label_frequency(&self) -> Node;

    fn neighbor_label_frequency(&self, node: Node) -> &HashMap<Node, Node>;
}

pub trait InputCapabilities {
    type GraphInput;
}

pub fn create_graph<G: From<EdgeList>>(edge_list: EdgeList) -> G {
    G::from(edge_list)
}

pub fn read_graph<G, F, P>(path: P, _fmt: F) -> Result<G, Error>
where
    P: AsRef<Path>,
    F: InputCapabilities,
    for<'a> F::GraphInput: From<&'a Path>,
    G: From<F::GraphInput>,
{
    Ok(G::from(F::GraphInput::from(path.as_ref())))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        graph::{DirectedCSRGraph, NodeLabeledCSRGraph, UndirectedCSRGraph},
        input::{DotGraphInput, EdgeListInput},
    };

    use super::*;

    #[test]
    fn read_graph_test() {
        let _g0: DirectedCSRGraph = read_graph("graph", EdgeListInput).unwrap();
        let _g1: UndirectedCSRGraph = read_graph("graph", EdgeListInput).unwrap();
        let _g2: NodeLabeledCSRGraph<DirectedCSRGraph> =
            read_graph("graph", DotGraphInput).unwrap();
        let _g3: NodeLabeledCSRGraph<UndirectedCSRGraph> =
            read_graph("graph", DotGraphInput).unwrap();
    }

    #[test]
    fn directed_graph_from_edge_list() {
        let edge_list = EdgeList::new(vec![(0, 1), (0, 2)]);

        let g: DirectedCSRGraph = create_graph(edge_list);

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);

        assert_eq!(g.out_degree(0), 2);
        assert_eq!(g.out_degree(1), 0);
        assert_eq!(g.out_degree(2), 0);

        assert_eq!(g.in_degree(0), 0);
        assert_eq!(g.in_degree(1), 1);
        assert_eq!(g.in_degree(2), 1);

        assert_eq!(g.out_neighbors(0), &[1, 2]);
        assert_eq!(g.out_neighbors(1), &[]);
        assert_eq!(g.out_neighbors(2), &[]);

        assert_eq!(g.in_neighbors(0), &[]);
        assert_eq!(g.in_neighbors(1), &[0]);
        assert_eq!(g.in_neighbors(2), &[0]);
    }

    #[test]
    fn undirected_graph_from_edge_list() {
        let edge_list = EdgeList::new(vec![(0, 1), (0, 2)]);

        let g: UndirectedCSRGraph = create_graph(edge_list);

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);

        assert_eq!(g.degree(0), 2);
        assert_eq!(g.degree(1), 1);
        assert_eq!(g.degree(2), 1);

        assert_eq!(g.neighbors(0), &[1, 2]);
        assert_eq!(g.neighbors(1), &[0]);
        assert_eq!(g.neighbors(2), &[0]);
    }

    #[test]
    fn directed_graph_from_edge_list_file() {
        let path = [env!("CARGO_MANIFEST_DIR"), "resources", "test.el"]
            .iter()
            .collect::<PathBuf>();

        let g: DirectedCSRGraph = read_graph(path, EdgeListInput).unwrap();

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);

        assert_eq!(g.out_degree(0), 2);
        assert_eq!(g.out_degree(1), 0);
        assert_eq!(g.out_degree(2), 0);

        assert_eq!(g.in_degree(0), 0);
        assert_eq!(g.in_degree(1), 1);
        assert_eq!(g.in_degree(2), 1);

        assert_eq!(g.out_neighbors(0), &[1, 2]);
        assert_eq!(g.out_neighbors(1), &[]);
        assert_eq!(g.out_neighbors(2), &[]);

        assert_eq!(g.in_neighbors(0), &[]);
        assert_eq!(g.in_neighbors(1), &[0]);
        assert_eq!(g.in_neighbors(2), &[0]);
    }

    #[test]
    fn undirected_graph_from_edge_list_file() {
        let path = [env!("CARGO_MANIFEST_DIR"), "resources", "test.el"]
            .iter()
            .collect::<PathBuf>();

        let g: UndirectedCSRGraph = read_graph(path, EdgeListInput).unwrap();

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);

        assert_eq!(g.degree(0), 2);
        assert_eq!(g.degree(1), 1);
        assert_eq!(g.degree(2), 1);

        assert_eq!(g.neighbors(0), &[1, 2]);
        assert_eq!(g.neighbors(1), &[0]);
        assert_eq!(g.neighbors(2), &[0]);
    }
}
