#![allow(clippy::unwrap_used)]
#![allow(clippy::indexing_slicing)]

use crate::map_helpers::safe_adder_set;
use crate::test_helpers::ToTest;
use petgraph::algo::{is_cyclic_directed, toposort};
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use petgraph::visit::{Dfs, Reversed};
use petgraph::Graph;
use quary_proto::test::TestType;
use quary_proto::{Model, Project, Test};
use std::collections::{HashMap, HashSet};

/// Edge represents an edge with (from, to) node names.
pub type Edge = (String, String);

pub struct ProjectGraph {
    pub edges: Vec<Edge>,
    pub graph: QGraph,
}

pub fn project_to_graph(project: Project) -> Result<ProjectGraph, String> {
    let mut taken: HashSet<String> = HashSet::new();
    let mut edges: Vec<Edge> = Vec::new();

    for name in project.seeds.keys() {
        safe_adder_set(&mut taken, name.clone())?;
    }

    for name in project.sources.keys() {
        safe_adder_set(&mut taken, name.clone())?;
    }

    let mut models: HashMap<String, Model> = HashMap::new();
    for (name, model) in &project.models {
        safe_adder_set(&mut taken, name.clone())?;
        models.insert(model.name.clone(), model.clone());
    }
    for (name, model) in &project.models {
        for reference in &model.references {
            if !taken.contains(reference) {
                return Err(format!(
                    "reference to {} in model {} does not exist in reference-able objects {}",
                    reference,
                    name,
                    Vec::from_iter(taken).join(", "),
                ));
            };
            edges.push((reference.clone(), model.name.clone()))
        }
    }

    let mut tests: HashMap<String, Test> = HashMap::new();
    for (name, test) in &project.tests {
        match &test.test_type {
            Some(TestType::Sql(test)) => {
                safe_adder_set(&mut taken, name.clone())?;

                tests.insert(name.clone(), test.to_test());

                for reference in test.references.clone() {
                    if !taken.contains(&reference) {
                        return Err(format!(
                            "reference to {} in model {} does not exist in reference-able objects {}",
                            reference,
                            name,
                            Vec::from_iter(taken).join(","),
                        ));
                    };
                    edges.push((reference, name.clone()))
                }
            }
            Some(TestType::NotNull(test)) => {
                safe_adder_set(&mut taken, name.clone())?;
                edges.push((test.model.clone(), name.clone()));
            }
            Some(TestType::Unique(test)) => {
                safe_adder_set(&mut taken, name.clone())?;
                edges.push((test.model.clone(), name.clone()));
            }
            Some(TestType::Relationship(test)) => {
                safe_adder_set(&mut taken, name.clone())?;
                edges.push((test.source_model.clone(), name.clone()));
                edges.push((test.target_model.clone(), name.clone()));
            }
            Some(TestType::AcceptedValues(test)) => {
                safe_adder_set(&mut taken, name.clone())?;
                edges.push((test.model.clone(), name.clone()));
            }
            Some(TestType::GreaterThanOrEqual(test)) => {
                safe_adder_set(&mut taken, name.clone())?;
                edges.push((test.model.clone(), name.clone()));
            }
            Some(TestType::LessThanOrEqual(test)) => {
                safe_adder_set(&mut taken, name.clone())?;
                edges.push((test.model.clone(), name.clone()));
            }
            Some(TestType::LessThan(test)) => {
                safe_adder_set(&mut taken, name.clone())?;
                edges.push((test.model.clone(), name.clone()));
            }
            Some(TestType::GreaterThan(test)) => {
                safe_adder_set(&mut taken, name.clone())?;
                edges.push((test.model.clone(), name.clone()));
            }
            _ => return Err(format!("unrecognised test type {:?}", test)),
        }
    }

    let graph = QGraph::new_from_nodes_and_edges(taken.clone(), edges.clone())?;

    Ok(ProjectGraph { edges, graph })
}

pub struct QGraph {
    dictionary: HashMap<String, NodeIndex>,
    pub graph: Graph<String, ()>,
}

impl QGraph {
    /// new_from_nodes_and_edges returns an error if the graph is cyclic and if any of the edges
    /// reference nodes that are not in the nodes set.
    fn new_from_nodes_and_edges(nodes: HashSet<String>, edges: Vec<Edge>) -> Result<Self, String> {
        let mut graph = Graph::<String, ()>::new();
        let mut dictionary = HashMap::<String, NodeIndex>::new();
        for node in &nodes {
            dictionary
                .entry(node.clone())
                .or_insert_with(|| graph.add_node(node.clone()));
        }
        for (from, to) in edges {
            if !nodes.contains(&from) {
                return Err(format!("node {} not found in nodes", from));
            }
            if !nodes.contains(&to) {
                return Err(format!("node {} not found in nodes", to));
            }
            let from_node: NodeIndex = *dictionary
                .entry(from.clone())
                .or_insert_with(|| graph.add_node(from.clone()));
            let to_node: NodeIndex = *dictionary
                .entry(to.clone())
                .or_insert_with(|| graph.add_node(to.clone()));

            graph.add_edge(from_node, to_node, ());
        }
        if is_cyclic_directed(&graph) {
            return Err("graph is cyclic".to_string());
        }
        Ok(QGraph { graph, dictionary })
    }

    pub fn to_dot_vis(&self) -> String {
        format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }

    /// Returns the node index of the reference.
    fn get_node(&self, node: &str) -> Option<NodeIndex> {
        let v = self.dictionary.get(node);
        v.copied()
    }

    /// return_parent_nods_to_apply_in_order returns the parents to the end in a vector where the
    /// models could be applied in the right order.
    pub fn return_parent_nods_to_apply_in_order(
        &self,
        end: &str,
    ) -> Result<(QGraph, Vec<String>), String> {
        let sub_graph = self.return_sub_graph(end)?;
        let upstream = sub_graph.return_upstream_graph(end)?;
        let sorted = upstream.get_node_sorted()?;
        Ok((upstream, sorted))
    }

    /// return_shrunk_downstream_graph returns the subgraph where the node override_name is the only
    /// node that is not removed upstream from the override_name. It ignores any nodes that are still
    /// connected through other nodes to the upstream graph.
    ///
    /// For example:
    ///   B -> C -> D and B -> D, then the graph will be shrunk to B -> D and C -> D
    ///
    /// In addition to the graph, the function returns a set of the nodes that were removed and or
    /// is the override name.
    pub fn return_shrunk_downstream_graph(
        &self,
        override_name: &str,
        target: &str,
    ) -> Result<(QGraph, HashSet<String>), String> {
        // Check if override_name exists in the graph
        if !self.dictionary.contains_key(override_name) {
            return Err("override_name not found in graph".to_string());
        }
        let mut new_graph = self.graph.clone();

        // Remove the upstream edges from the override_name
        let node_index = self
            .get_node(override_name)
            .ok_or(format!("could not find node for {}", override_name))?;
        let edges_collected = new_graph
            .edges_directed(node_index, petgraph::Direction::Incoming)
            .collect::<Vec<_>>()
            .iter()
            .map(|edge| edge.id().clone())
            .collect::<Vec<_>>();
        for edge in edges_collected {
            new_graph.remove_edge(edge);
        }

        // Start from the target and go upstream to find all the nodes that are still connected to the
        // target. These nodes are the ones that are not removed.
        let target_index = self
            .get_node(target)
            .ok_or(format!("could not find node for {}", override_name))?;
        let mut nodes_to_keep = HashSet::from([target.to_string()]);

        let reversed_graph = Reversed(&new_graph);
        let mut upstream_dfs = Dfs::new(&reversed_graph, target_index);
        while let Some(nx) = upstream_dfs.next(&reversed_graph) {
            nodes_to_keep.insert(new_graph[nx].clone());
        }

        // Collect all nodes that are not in the nodes_to_keep set
        let nodes_to_remove: Vec<String> = new_graph
            .node_indices()
            .filter(|&n| !nodes_to_keep.contains(&new_graph[n].clone()))
            .map(|node| {
                let name = new_graph.node_weight(node).unwrap();
                name.clone()
            })
            .collect::<Vec<_>>();

        // Remove the nodes that are not part of the upstream graph from the target
        let mut removed_node_names = HashSet::new();
        for node in nodes_to_remove.into_iter() {
            let node_index = new_graph
                .node_indices()
                .find(|index| *new_graph.node_weight(*index).unwrap() == node)
                .unwrap();
            let name = new_graph.node_weight(node_index).unwrap();
            removed_node_names.insert(name.to_string());
            new_graph.remove_node(node_index);
        }

        let dictionary = new_graph
            .node_indices()
            .map(|index| (new_graph.node_weight(index).unwrap().clone(), index))
            .collect::<HashMap<String, NodeIndex>>();

        Ok((
            QGraph {
                graph: new_graph,
                dictionary,
            },
            removed_node_names,
        ))
    }

    /// return_graph_edges returns the graph edges in the right names
    pub fn return_graph_edges(&self) -> Result<Vec<Edge>, String> {
        self.graph
            .raw_edges()
            .iter()
            .map(|edge| {
                let source = self
                    .get_node_name(&edge.source())
                    .ok_or("could not find source")?;
                let target = self
                    .get_node_name(&edge.target())
                    .ok_or("could not find target")?;
                Ok((source, target))
            })
            .collect()
    }

    /// return_sub_graph returns the nodes and edges that are connected regardless of connection direction
    /// to a given node in a sub graph of the total graph.
    pub fn return_sub_graph(&self, start: &str) -> Result<QGraph, String> {
        let start_node = self
            .get_node(start)
            .ok_or(format!("could not find node for {}", start))?;

        let mut visited = vec![false; self.graph.node_count()];
        let mut subgraph: Graph<String, ()> = Graph::new();
        let mut node_map = HashMap::new();

        // Convert the graph to an undirected version for traversal
        let mut undirected_graph = self.graph.clone();
        undirected_graph.clear_edges();

        for edge in self.graph.raw_edges() {
            undirected_graph.add_edge(edge.source(), edge.target(), ());
            undirected_graph.add_edge(edge.target(), edge.source(), ());
        }

        self.undirected_dfs(
            &mut visited,
            start_node,
            &mut node_map,
            &mut subgraph,
            &undirected_graph,
        );

        let dictionary = node_map
            .iter()
            .map(|(k, v)| (self.get_node_name(k).unwrap(), *v))
            .collect::<HashMap<String, NodeIndex>>();
        let node_hash_set = dictionary.keys().cloned().collect::<HashSet<String>>();

        let mut new_graph = self.graph.clone();
        loop {
            let edge_to_remove = new_graph.edge_indices().find(|edge| {
                let (source, target) = new_graph.edge_endpoints(*edge).unwrap();
                let source = self.get_node_name(&source).unwrap();
                let target = self.get_node_name(&target).unwrap();
                !(node_hash_set.contains(&source) || node_hash_set.contains(&target))
            });
            if let Some(edge) = edge_to_remove {
                new_graph.remove_edge(edge).unwrap();
            } else {
                break;
            }
        }
        loop {
            let nodes_to_remove = new_graph.node_indices().find(|index| {
                let node = new_graph.node_weight(*index).unwrap();
                !node_hash_set.contains(&node.clone())
            });
            if let Some(node) = nodes_to_remove {
                new_graph.remove_node(node).ok_or("could not remove node")?;
            } else {
                break;
            }
        }
        let out_dictionary = new_graph.node_indices().fold(
            HashMap::new(),
            |mut acc: HashMap<String, NodeIndex>, index| {
                let node = new_graph.node_weight(index).unwrap().clone();
                acc.insert(node, index);
                acc
            },
        );

        Ok(QGraph {
            graph: new_graph,
            dictionary: out_dictionary,
        })
    }

    #[allow(clippy::indexing_slicing)]
    fn undirected_dfs(
        &self,
        visited: &mut Vec<bool>,
        current_node: NodeIndex,
        node_map: &mut HashMap<NodeIndex, NodeIndex>,
        subgraph: &mut Graph<String, ()>,
        undirected_graph: &Graph<String, ()>,
    ) {
        visited[current_node.index()] = true;

        let new_index = subgraph.add_node(self.get_node_name(&current_node).unwrap());
        node_map.insert(current_node, new_index);

        for neighbor in undirected_graph.neighbors(current_node) {
            if !visited.get(neighbor.index()).unwrap() {
                self.undirected_dfs(visited, neighbor, node_map, subgraph, undirected_graph);

                // Ensure both nodes exist in the subgraph before adding an edge
                if node_map.contains_key(&neighbor) {
                    let neighbor_index_in_subgraph = *node_map.get(&neighbor).unwrap();
                    subgraph.add_edge(new_index, neighbor_index_in_subgraph, ());
                }
            }
        }
    }

    /// upstream graph returns the subgraph that is upstream from the the start. Such that it
    /// includes all that parents that feed into the start, including the start.
    pub fn return_upstream_graph(&self, start: &str) -> Result<QGraph, String> {
        let start_node = match self.dictionary.get(start) {
            Some(node) => *node,
            None => return Err(format!("Node {} not found in graph", start)),
        };

        // Reverse the graph
        let mut reversed_graph = self.graph.clone();
        reversed_graph.clear_edges();
        for edge in self.graph.raw_edges() {
            reversed_graph.add_edge(edge.target(), edge.source(), ());
        }

        let mut dfs = Dfs::new(&reversed_graph, start_node);

        // Create an empty graph and dictionary for the subgraph
        let mut subgraph = QGraph {
            graph: Graph::new(),
            dictionary: HashMap::new(),
        };

        while let Some(node) = dfs.next(&reversed_graph) {
            // Add the node to the subgraph and store the new index in the dictionary
            #[allow(clippy::indexing_slicing)]
            let node_data = self.graph[node].clone();
            let new_index = subgraph.graph.add_node(node_data.clone());
            subgraph.dictionary.insert(node_data, new_index);

            for parent in reversed_graph.neighbors(node) {
                #[allow(clippy::indexing_slicing)]
                let parent_data = self.graph[parent].clone();
                if let Some(parent_index) = subgraph.dictionary.get(&parent_data) {
                    // If the node has a parent in the DFS tree, add an edge in the subgraph
                    subgraph.graph.add_edge(new_index, *parent_index, ());
                }
            }
        }

        let nodes_to_keep = subgraph.graph.node_weights().collect::<HashSet<_>>();
        let mut new_graph = self.graph.clone();
        loop {
            let edge_to_remove = new_graph.edge_indices().find(|edge| {
                let (source, target) = new_graph.edge_endpoints(*edge).unwrap();
                let source = self.get_node_name(&source).unwrap();
                let target = self.get_node_name(&target).unwrap();
                !nodes_to_keep.contains(&source) || !nodes_to_keep.contains(&target)
            });
            if let Some(edge) = edge_to_remove {
                new_graph.remove_edge(edge).unwrap();
            } else {
                break;
            }
        }
        loop {
            let nodes_to_remove = new_graph.node_indices().find(|index| {
                let node = new_graph.node_weight(*index).unwrap();
                !nodes_to_keep.contains(&node.clone())
            });
            if let Some(node) = nodes_to_remove {
                new_graph.remove_node(node).unwrap();
            } else {
                break;
            }
        }
        let out_dictionary = new_graph.node_indices().fold(
            HashMap::new(),
            |mut acc: HashMap<String, NodeIndex>, index| {
                let node = new_graph.node_weight(index).unwrap().clone();
                acc.insert(node, index);
                acc
            },
        );

        Ok(QGraph {
            graph: new_graph,
            dictionary: out_dictionary,
        })
    }

    /// Returns the nodes in the graph in a sorted order so that they can be applied in the right order.
    /// The order is determined by the order of the nodes in the graph.
    ///
    /// TODO Make this deterministic by sorting the nodes by name.
    pub fn get_node_sorted(&self) -> Result<Vec<String>, String> {
        match toposort(&self.graph, None) {
            Ok(nodes) => {
                // Create the reverse dictionary for looking up node names by NodeIndex
                let outs = nodes
                    .iter()
                    .map(|out| {
                        self.get_node_name(out)
                            .ok_or_else(|| "Node not found in dictionary".to_string())
                    })
                    .collect::<Result<Vec<String>, String>>()?;

                Ok(outs)
            }
            Err(_) => Err("Cycle detected in graph during topological sort".to_string()),
        }
    }

    /// Returns the name of the node at the given index.
    pub fn get_node_name(&self, index: &NodeIndex) -> Option<String> {
        let dict: HashMap<&NodeIndex, String> = self
            .dictionary
            .iter()
            .map(|(k, v)| (v, k.to_string()))
            .collect();
        if let Some(v) = dict.get(index) {
            return Some(v.to_string());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_sqlite::DatabaseQueryGeneratorSqlite;
    use crate::project::parse_project;
    use prost::bytes::Bytes;
    use quary_proto::FileSystem;

    #[test]
    fn test_get_node_sorted() {
        let tests = vec![
            ("empty", vec![], vec![], false),
            (
                "simple diagram",
                vec![
                    ("B".to_string(), "C".to_string()),
                    ("A".to_string(), "B".to_string()),
                ],
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
                false,
            ),
            // TODO Reinstate and make the function deterministic
            // (
            //     "diamond",
            //     vec![
            //         ["A".to_string(), "B".to_string()],
            //         ["A".to_string(), "C".to_string()],
            //         ["B".to_string(), "D".to_string()],
            //         ["C".to_string(), "D".to_string()],
            //     ],
            //     vec![
            //         "A".to_string(),
            //         "B".to_string(),
            //         "C".to_string(),
            //         "D".to_string(),
            //     ],
            //     false,
            // ), // TODO Add more complex tests
            // (
            //     "diamond",
            //     vec![
            //         ["A".to_string(), "B".to_string()],
            //         ["A".to_string(), "C".to_string()],
            //         ["Z".to_string(), "B".to_string()],
            //         ["X".to_string(), "C".to_string()],
            //         ["B".to_string(), "D".to_string()],
            //         ["C".to_string(), "D".to_string()],
            //     ],
            //     vec![
            //         "A".to_string(),
            //         "X".to_string(),
            //         "Z".to_string(),
            //         "B".to_string(),
            //         "C".to_string(),
            //         "D".to_string(),
            //     ],
            //     false,
            // ),
            // (
            //     "pyramid",
            //     vec![
            //         ["A".to_string(), "D".to_string()],
            //         ["B".to_string(), "D".to_string()],
            //         ["B".to_string(), "E".to_string()],
            //         ["C".to_string(), "E".to_string()],
            //         ["D".to_string(), "F".to_string()],
            //         ["E".to_string(), "F".to_string()],
            //     ],
            //     vec![
            //         "A".to_string(),
            //         "B".to_string(),
            //         "C".to_string(),
            //         "D".to_string(),
            //         "E".to_string(),
            //         "F".to_string(),
            //     ],
            //     false,
            // ),
        ];

        for (name, edges, want, want_err) in tests {
            println!("Running test: {}", name);

            let nodes = edges
                .iter()
                .flat_map(|(a, b)| vec![a.clone(), b.clone()])
                .collect::<HashSet<String>>();
            let g = QGraph::new_from_nodes_and_edges(nodes, edges).unwrap();
            let got = match g.get_node_sorted() {
                Ok(got) => got,
                Err(e) => {
                    assert!(want_err, "Unexpected error: {:?}", e);
                    continue;
                }
            };
            assert!(!want_err, "Expected error, but none occurred.");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_get_node_get_name() {
        let tests = vec![
            ("empty", vec![], "C", false),
            (
                "simple diagram",
                vec![("B".to_string(), "C".to_string())],
                "B",
                true,
            ),
            (
                "simple diagram, target",
                vec![("B".to_string(), "C".to_string())],
                "C",
                true,
            ),
        ];

        for (name, edges, node_name, want_value) in tests {
            println!("Running test: {}", name);

            let nodes = edges
                .iter()
                .flat_map(|(a, b)| vec![a.clone(), b.clone()])
                .collect::<HashSet<String>>();
            let g = QGraph::new_from_nodes_and_edges(nodes, edges).unwrap();
            let got = g.get_node(node_name);

            if !want_value {
                assert_eq!(got, None);
            } else {
                let got_back = g.get_node_name(&got.unwrap()).unwrap();

                assert_eq!(got_back, node_name.to_string());
            }
        }
    }

    #[test]
    fn test_to_dot_vis() {
        let tests: Vec<(&str, Vec<Edge>, &str)> = vec![
            ("empty", vec![], "digraph {\n}\n"),
            // TODO Implement the tests below
            // (
            //     "simple diagram",
            //     vec![["B".to_string(), "C".to_string()]],
            //     "digraph {\n    0 [ label = \"\\\"B\\\"\" ]\n    1 [ label = \"\\\"C\\\"\" ]\n    0 -> 1 [ ]\n}\n}",
            // ),
            // (
            //     "diamond",
            //     vec![
            //         ["A".to_string(), "B".to_string()],
            //         ["A".to_string(), "C".to_string()],
            //         ["B".to_string(), "D".to_string()],
            //         ["C".to_string(), "D".to_string()],
            //     ],
            //     "digraph {\n\t\"A\" -> \"B\"\n\t\"A\" -> \"C\"\n\t\"B\" -> \"D\"\n\t\"C\" -> \"D\"\n}",
            // ),
        ];

        for (name, edges, want) in tests {
            println!("Running test: {}", name);

            let nodes = edges
                .iter()
                .flat_map(|(a, b)| vec![a.clone(), b.clone()])
                .collect::<HashSet<String>>();

            let g = QGraph::new_from_nodes_and_edges(nodes, edges).unwrap();
            let got = g.to_dot_vis();

            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_return_sub_graph() {
        let tests = vec![
            (
                "simple",
                vec![("a".to_string(), "b".to_string())],
                "a",
                vec!["a", "b"],
                1,
                vec![("a".to_string(), "b".to_string())],
            ),
            (
                "more complex",
                vec![
                    ("a".to_string(), "b".to_string()),
                    ("a".to_string(), "c".to_string()),
                    ("c".to_string(), "d".to_string()),
                    ("b".to_string(), "d".to_string()),
                    ("x".to_string(), "y".to_string()),
                ],
                "a",
                vec!["a", "b", "c", "d"],
                4,
                vec![
                    ("a".to_string(), "b".to_string()),
                    ("a".to_string(), "c".to_string()),
                    ("c".to_string(), "d".to_string()),
                    ("b".to_string(), "d".to_string()),
                ],
            ),
            (
                "more complex",
                vec![
                    ("a".to_string(), "b".to_string()),
                    ("a".to_string(), "c".to_string()),
                    ("c".to_string(), "d".to_string()),
                    ("b".to_string(), "d".to_string()),
                    ("x".to_string(), "y".to_string()),
                ],
                "b",
                vec!["a", "b", "c", "d"],
                4,
                vec![
                    ("a".to_string(), "b".to_string()),
                    ("a".to_string(), "c".to_string()),
                    ("c".to_string(), "d".to_string()),
                    ("b".to_string(), "d".to_string()),
                ],
            ),
        ];

        for (name, edges, search, want, edges_want_length, edges_want) in tests {
            println!("Running test: {}", name);

            let nodes = edges
                .iter()
                .flat_map(|(a, b)| vec![a.clone(), b.clone()])
                .collect::<HashSet<String>>();

            let g = QGraph::new_from_nodes_and_edges(nodes, edges).unwrap();
            let got = g.return_sub_graph(search).unwrap();

            let mut values = got
                .graph
                .node_indices()
                .map(|node| got.get_node_name(&node).unwrap())
                .collect::<Vec<String>>();
            values.sort();

            let mut want_set = HashSet::new();
            edges_want.iter().for_each(|edge| {
                want_set.insert(edge.clone());
            });
            let mut got_set = HashSet::new();
            got.return_graph_edges().unwrap().iter().for_each(|edge| {
                got_set.insert(edge.clone());
            });

            assert_eq!(
                want_set
                    .iter()
                    .map(|(a, b)| (a.to_string(), b.to_string()))
                    .collect::<HashSet<_>>(),
                got_set
            );
            assert_eq!(got.return_graph_edges().unwrap().len(), edges_want_length);
            assert_eq!(values, want);
        }
    }

    #[test]
    fn test_return_sub_graph_single_node() {
        let fs = FileSystem {
            files: vec![
                ("quary.yaml", "sqliteInMemory: {}"),
                ("models/test_model.sql", "SELECT \"1\""),
            ]
            .iter()
            .map(|(name, content)| {
                (
                    name.to_string(),
                    quary_proto::File {
                        name: name.to_string(),
                        contents: Bytes::from(content.to_string()),
                    },
                )
            })
            .collect(),
        };
        let db = DatabaseQueryGeneratorSqlite::default();

        let project = parse_project(&fs, &db, "").unwrap();

        let graph = project_to_graph(project).unwrap();

        graph.graph.return_sub_graph("test_model").unwrap();
    }

    #[test]
    fn test_return_upstream_graph() {
        let tests = vec![
            // TODO Implement these tests
            // ("empty", "a", vec![], vec![]),
            ("single_node", "a", vec![("a", "b")], vec!["a"], 0),
            ("two_nodes", "b", vec![("a", "b")], vec!["a", "b"], 1),
            (
                "diamond",
                "d",
                vec![("a", "b"), ("a", "c"), ("c", "d"), ("b", "d"), ("d", "e")],
                vec!["a", "b", "c", "d"],
                4,
            ),
            (
                "diamond with end",
                "e",
                vec![("a", "b"), ("a", "c"), ("c", "d"), ("b", "d"), ("d", "e")],
                vec!["a", "b", "c", "d", "e"],
                5,
            ),
            // ("two_nodes", "b", vec![["a", "b"]], vec!["a", "b"]),
        ];

        for (name, search, edges, want, expected_edges_length) in tests {
            println!("Running test: {}", name);

            let edges: Vec<Edge> = edges
                .into_iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();
            let nodes = edges
                .iter()
                .flat_map(|(a, b)| vec![a.clone(), b.clone()])
                .collect::<HashSet<String>>();

            let g = QGraph::new_from_nodes_and_edges(nodes, edges).unwrap();

            let got = g.return_upstream_graph(search).unwrap();

            assert_eq!(got.dictionary.len(), want.len());
            assert_eq!(expected_edges_length, got.graph.edge_count());
        }
    }

    #[test]
    fn test_return_parent_nods_to_apply_in_order() {
        let tests = vec![
            // ("empty", "a", vec![], vec![]),
            ("single_node", "a", vec![["a", "b"]], vec!["a"]),
            ("two_nodes", "b", vec![["a", "b"]], vec!["a", "b"]),
            (
                "diamond",
                "b",
                vec![["a", "b"], ["a", "c"], ["c", "d"], ["b", "d"], ["x", "y"]],
                vec!["a", "b"],
            ),
            ("two_nodes", "b", vec![["a", "b"]], vec!["a", "b"]),
        ];

        for (name, search, edges, want) in tests {
            println!("Running test: {}", name);
            let edges: Vec<Edge> = edges
                .into_iter()
                .map(|[a, b]| (a.to_string(), b.to_string()))
                .collect();
            let nodes = edges
                .iter()
                .flat_map(|(a, b)| vec![a.clone(), b.clone()])
                .collect::<HashSet<String>>();
            let g = QGraph::new_from_nodes_and_edges(nodes, edges).unwrap();

            let (_, got) = g.return_parent_nods_to_apply_in_order(search).unwrap();

            assert_eq!(got, want);
        }
    }

    #[test]
    fn return_overriden_downstream_graph() {
        struct Test<'a> {
            name: &'a str,
            target: &'a str,
            nodes: HashSet<&'a str>,
            edges: Vec<(&'a str, &'a str)>,
            override_name: &'a str,
            expect_err: bool,
            expect_nodes: HashSet<&'a str>,
            expect_edges: Vec<(&'a str, &'a str)>,
            expect_removed: HashSet<&'a str>,
        }

        let tests = [
            Test {
                name: "empty",
                target: "a",
                edges: vec![],
                nodes: HashSet::new(),
                override_name: "a",
                expect_err: true,
                expect_nodes: HashSet::new(),
                expect_edges: vec![],
                expect_removed: HashSet::from([]),
            },
            Test {
                name: "single_node",
                target: "a",
                edges: vec![],
                nodes: HashSet::from(["a"]),
                override_name: "a",
                expect_err: false,
                expect_nodes: HashSet::from(["a"]),
                expect_edges: vec![],
                expect_removed: HashSet::from([]),
            },
            Test {
                name: "single edge",
                nodes: HashSet::from(["a", "b"]),
                edges: vec![("a", "b")],
                target: "b",
                override_name: "a",
                expect_err: false,
                expect_nodes: HashSet::from(["a", "b"]),
                expect_edges: vec![("a", "b")],
                expect_removed: HashSet::from([]),
            },
            Test {
                name: "one node removal",
                target: "b",
                nodes: HashSet::from(["a", "b", "z"]),
                edges: vec![("z", "a"), ("a", "b")],
                override_name: "a",
                expect_err: false,
                expect_nodes: HashSet::from(["a", "b"]),
                expect_edges: vec![("a", "b")],
                expect_removed: HashSet::from(["z"]),
            },
            Test {
                name: "tree removal",
                nodes: HashSet::from(["a", "b", "z", "x"]),
                edges: vec![("a", "b"), ("z", "a"), ("x", "a")],
                target: "b",
                override_name: "a",
                expect_err: false,
                expect_nodes: HashSet::from(["a", "b"]),
                expect_edges: vec![("a", "b")],
                expect_removed: HashSet::from(["z", "x"]),
            },
            Test {
                name: "loop",
                target: "b",
                nodes: HashSet::from(["a", "b", "z", "x", "k"]),
                edges: vec![("a", "b"), ("z", "a"), ("z", "x"), ("x", "b"), ("k", "x")],
                override_name: "a",
                expect_err: false,
                expect_nodes: HashSet::from(["a", "b", "z", "x", "k"]),
                expect_edges: vec![("a", "b"), ("z", "x"), ("x", "b"), ("k", "x")],
                expect_removed: HashSet::from([]),
            },
        ];

        for test in tests.iter() {
            println!("Running test: {}", test.name);
            let edges: Vec<Edge> = test
                .edges
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();

            let g = QGraph::new_from_nodes_and_edges(
                test.nodes.iter().map(|s| s.to_string()).collect(),
                edges,
            )
            .unwrap();

            let got = g.return_shrunk_downstream_graph(test.override_name, test.target);

            assert_eq!(got.is_err(), test.expect_err);
            if let Ok((got, removed)) = got {
                assert_eq!(
                    got.graph.node_count(),
                    test.expect_nodes.len(),
                    "node count mismatch"
                );
                assert_eq!(
                    removed,
                    test.expect_removed
                        .iter()
                        .map(|&s| s.to_string())
                        .collect::<HashSet<_>>()
                );
                assert_eq!(
                    got.return_graph_edges()
                        .unwrap()
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect::<HashSet<_>>(),
                    test.expect_edges
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect::<HashSet<_>>()
                );
            }
        }
    }
}
