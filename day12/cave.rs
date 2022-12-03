use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
enum Node {
	Start,
	End,
	Small(String),
	Large(String)
}

impl PartialEq<String> for Node {
	fn eq(&self, other: &String) -> bool {
		return match self {
			Node::Start => other == "start",
			Node::End => other == "end",
			Node::Small(v) => other == v,
			Node::Large(v) => other == v
		};
	}
}

impl From<String> for Node {
	fn from(value: String) -> Self {
		if value == "start" {
			return Node::Start;
		}
		if value == "end" {
			return Node::End;
		}
		if value.chars().all(|c| c.is_lowercase()) {
			return Node::Small(value);
		}
		return Node::Large(value)
	}
}

pub struct GraphBuilder {
	nodes: Vec<Node>,
	edges: Vec<HashSet<usize>>
}

impl GraphBuilder {
	pub fn new() -> Self {
		return GraphBuilder { nodes: Vec::new(), edges: Vec::new() };
	}

	pub fn add_edge(&mut self, from: String, to: String) {
		let from_node = self.insert_or_get_node(from);
		let to_node = self.insert_or_get_node(to);

		self.edges[from_node].insert(to_node);
		self.edges[to_node].insert(from_node);
	}

	fn insert_or_get_node(&mut self, node_name: String) -> usize {
		return self.nodes.iter()
			.position(|e| *e == node_name)
			.unwrap_or_else(|| {
				self.nodes.push(Node::from(node_name));
				self.edges.push(HashSet::new());
				return self.nodes.len() - 1;
			});
	}

	pub fn build(self) -> Graph {
		return Graph { nodes: self.nodes, edges: self.edges };
	}
}

#[derive(Debug)]
pub struct Graph {
	nodes: Vec<Node>,
	edges: Vec<HashSet<usize>>
}

impl Graph {
	pub fn builder() -> GraphBuilder {
		return GraphBuilder::new();
	}

	pub fn paths(&self, visit_twice: bool) -> usize {
		let start_node_index = self.nodes.iter()
			.position(|e| *e == Node::Start).unwrap();
		return self.explore(visit_twice, &mut Vec::new(), start_node_index);
	}

	fn explore(&self, mut visit_twice: bool, visited: &mut Vec<usize>, node_index: usize) -> usize {
		let node = &self.nodes[node_index];

		if node == &Node::End {
			return 1;
		}
		if visited.contains(&node_index) {
			if node == &Node::Start {
				return 0;
			}
			if let Node::Small(_) = node {
				if !visit_twice {
					return 0;
				}
				visit_twice = false;
			}
		}

		visited.push(node_index);
		let paths = self.edges[node_index].iter()
			.map(|&a| self.explore(visit_twice, visited, a))
			.sum();
		visited.pop();

		return paths;
	}
}
