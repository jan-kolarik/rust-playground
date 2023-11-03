use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodeCost {
    cost: Cost,
    node: Node
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(graph: &Graph, start: Node, end: Node) -> (Vec<Node>, Cost) {
    let mut distances: HashMap<Node, Cost> = graph.nodes.iter().map(|&node| (node, usize::MAX)).collect();
    let mut parents: HashMap<Node, NodeCost> = HashMap::new();
    let mut queue: BinaryHeap<NodeCost> = BinaryHeap::new();

    distances.insert(start, 0);
    queue.push(NodeCost {cost: 0, node: start});

    while let Some(NodeCost {cost, node}) = queue.pop() {
        if node == end {
            break;
        }

        if cost > distances[&node] {
            continue;
        }

        for &(neighbor, weight) in &graph.edges[&node] {
            let new_cost = cost + weight;

            if new_cost < distances[&neighbor] {
                distances.insert(neighbor, new_cost);
                parents.insert(neighbor, NodeCost {cost: weight, node: node});
                queue.push(NodeCost{cost: new_cost, node: neighbor});
            }
        }
    }

    let mut cost = 0;
    let mut path = Vec::new();
    let mut current = NodeCost {cost: 0, node: end};

    while let Some(&parent) = parents.get(&current.node) {
        path.push(parent.node);
        cost += parent.cost;

        if parent.node == start {
            break;
        }

        current = parent;
    }

    if !path.is_empty() {
        path.reverse();
        path.push(end);
    } 

    (path, cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph() {
        assert_eq!((vec![], 0), shortest_path(&Graph {edges: HashMap::new(), nodes: HashSet::new()}, 0, 0));
    }

    #[test]
    fn one_node_graph() {
        let mut graph = Graph {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        };

        graph.nodes.insert(1);

        assert_eq!((vec![], 0), shortest_path(&graph, 1, 1));
    }

    #[test]
    fn one_edge_graph() {
        let mut graph = Graph {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        };

        graph.nodes.insert(1);
        graph.nodes.insert(2);
        graph.edges.insert(1, vec![(2, 1)]);

        assert_eq!((vec![1, 2], 1), shortest_path(&graph, 1, 2));
    }

    #[test]
    fn small_graph() {
        let mut graph = Graph {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        };

        graph.edges.insert(0, vec![(1, 4), (2, 2)]);
        graph.edges.insert(1, vec![(3, 5)]);
        graph.edges.insert(2, vec![(1, 1), (3, 8)]);
        graph.edges.insert(3, vec![]);

        graph.nodes.insert(0);
        graph.nodes.insert(1);
        graph.nodes.insert(2);
        graph.nodes.insert(3);

        assert_eq!((vec![0, 2, 1, 3], 8), shortest_path(&graph, 0, 3));
        assert_eq!((vec![], 0), shortest_path(&graph, 3, 0));
    }
}