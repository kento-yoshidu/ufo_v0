// graph

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, key: &str) {
        self.edges.entry(key.to_string()).or_insert_with(Vec::new);
    }

    pub fn add_edge(&mut self, key_a: &str, key_b: &str) {
        self.edges.entry(key_a.to_string()).or_insert_with(Vec::new).push(key_b.to_string());
        self.edges.entry(key_b.to_string()).or_insert_with(Vec::new).push(key_a.to_string());
    }

    pub fn remove_edge(&mut self, key_a: &str, key_b: &str) {
        let Some(map) = self.edges.get_mut(key_a) else {
            return;
            // Todo:
        };

        map.retain(|key| key != key_b);

        let Some(map) = self.edges.get_mut(key_b) else {
            return;
            // Todo:
        };

        map.retain(|key| key != key_a);
    }

    pub fn neighbors(&self, key: &str) -> Option<&Vec<String>> {
        self.edges.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph_has_no_edges() {
        let graph = Graph::new();

        assert_eq!(graph.edges.get("a"), None);
    }

    #[test]
    fn add_edge_registers_both_directions() {
        let mut graph = Graph::new();

        graph.add_edge("a", "b");

        assert_eq!(graph.edges.get("a"), Some(&vec!["b".to_string()]));
        assert_eq!(graph.edges.get("b"), Some(&vec!["a".to_string()]));
    }

    #[test]
    fn add_edge_appends_to_existing_neighbors() {
        let mut graph = Graph::new();

        graph.add_edge("a", "b");
        graph.add_edge("a", "c");

        assert_eq!(graph.edges.get("a"), Some(&vec!["b".to_string(), "c".to_string()]));
    }

    #[test]
    fn add_node_creates_empty_entry() {
        let mut graph = Graph::new();

        graph.add_node("a");

        assert_eq!(graph.edges.get("a"), Some(&vec![]));
    }

    #[test]
    fn add_node_does_not_clobber_existing_edges() {
        let mut graph = Graph::new();

        graph.add_edge("a", "b");
        graph.add_node("a");

        assert_eq!(graph.edges.get("a"), Some(&vec!["b".to_string()]));
    }

    #[test]
    fn neighbors_returns_none_for_unknown_key() {
        let graph = Graph::new();

        assert_eq!(graph.neighbors("a"), None);
    }

    #[test]
    fn remove_edge_deletes_both_directions() {
        let mut graph = Graph::new();

        graph.add_edge("a", "b");
        graph.remove_edge("a", "b");

        assert_eq!(graph.neighbors("a"), Some(&vec![]));
        assert_eq!(graph.neighbors("b"), Some(&vec![]));
    }

    #[test]
    fn remove_edge_keeps_other_neighbors() {
        let mut graph = Graph::new();

        graph.add_edge("a", "b");
        graph.add_edge("a", "c");

        graph.remove_edge("a", "b");

        assert_eq!(graph.neighbors("a"), Some(&vec!["c".to_string()]));
        assert_eq!(graph.neighbors("c"), Some(&vec!["a".to_string()]));
    }

    #[test]
    fn remove_edge_is_noop_for_missing_edge() {
        let mut graph = Graph::new();

        graph.add_edge("a", "b");
        graph.remove_edge("a", "c");

        assert_eq!(graph.neighbors("a"), Some(&vec!["b".to_string()]));
    }

    #[test]
    fn remove_edge_does_not_panic_for_unknown_keys() {
        let mut graph = Graph::new();

        graph.remove_edge("x", "y");

        assert_eq!(graph.neighbors("x"), None);
    }
}
