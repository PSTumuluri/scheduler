use std::collections::HashSet;

type AdjacencyList = HashSet<usize>;

/// Directed graph implemented as an adjacency list.
/// Each vertex is represented as an unsigned integer.
struct DirectedGraph {
    // A None entry indicates that the vertex does not exist.
    // This is different from a vertex with an empty adjacency list,
    // which *does* exist but simply has an outdegree of 0.
    adj_lists: Vec<Option<AdjacencyList>>,

    // Need to keep track of this separately because the length
    // of the list is actually more like a capacity, since None entries
    // will be included in it.
    num_vertices: usize,
}

impl DirectedGraph {
    /// Construct a new graph with the specified initial capacity.
    /// The graph still starts with no vertices, but using a sensible
    /// initial capacity prevents the need for frequent resizing of the
    /// adjacency list.
    fn new(capacity: usize) -> Self {
        let adj_lists = Vec::with_capacity(capacity);
        DirectedGraph {
            adj_lists,
            num_vertices: 0,
        }
    }

    /// Return the number of vertices in the graph.
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }
    
    /// Attempts to add a vertex at the specified index.
    /// Returns true if the operation succeeded and false otherwise
    fn add_vertex(&mut self, index: usize) -> bool {
        if index >= self.adj_lists.len() {
            self.adj_lists.resize(index + 1, None);
        }
        if self.adj_lists[index] == None {
            self.adj_lists.insert(index, Some(HashSet::new()));
            self.num_vertices += 1;
            true
        } else {
            false
        }
    }

    /// Create a directed edge from src to dest, returning Err if at least one
    /// of the verticies does not exist.
    pub fn add_edge(&mut self, src: usize, dest: usize) -> Result<(), &'static str> {
        if self.has_vertex(src) && self.has_vertex(dest) {
            self.adj_lists[src].as_mut().unwrap().insert(dest);
            return Ok(());
        }
        Err("one or more vertices do not exist")
    }
    
    /// Returns true if and only if the graph contains the specified vertex.
    fn has_vertex(&self, vertex: usize) -> bool {
        vertex < self.adj_lists.len() && matches!(self.adj_lists[vertex], Some(_))
    }

    /// Returns true if and only if an edge exists between src and dest.
    /// Returns an Err if either of the vertices is not in the graph.
    pub fn has_edge(&self, src: usize, dest: usize) -> Result<bool, &'static str> {
        if self.has_vertex(src) && self.has_vertex(dest) {
            let list = self.adj_lists[src].as_ref().unwrap();
            for &i in list.iter() {
                if i == dest {
                    return Ok(true);
                }
            }
            return Ok(false);
        }
        Err("src and/or dest vertices do not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_demo() {
        let mut g = DirectedGraph::new(10);
        assert_eq!(g.num_vertices(), 0);

        assert!(g.add_vertex(0));
        assert_eq!(g.num_vertices(), 1);

        assert!(g.add_vertex(3));
        assert_eq!(g.num_vertices(), 2);

        assert_eq!(g.adj_lists[0].as_ref().unwrap().len(), 0);
        assert_eq!(g.adj_lists[3].as_ref().unwrap().len(), 0);

        g.add_edge(0, 3);
        assert_eq!(g.adj_lists[0].as_ref().unwrap().len(), 1);
        assert_eq!(g.adj_lists[3].as_ref().unwrap().len(), 0);

        assert!(matches!(g.has_edge(0, 3), Ok(true)));
        assert!(matches!(g.has_edge(3, 0), Ok(false)));
        assert!(matches!(g.has_edge(0, 2), Err(_)));
    }
}
