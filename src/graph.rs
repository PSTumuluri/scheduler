use std::collections::{HashMap, HashSet, LinkedList};

pub type Vertex = usize;
type AdjacencyList = HashSet<Vertex>;

pub struct DirectedGraph {
    adj_lists: HashMap<Vertex, AdjacencyList>,
}

impl DirectedGraph {
    pub fn new() -> Self {
        Self {
            adj_lists: HashMap::new(),
        }
    }

    /// Add the vertex to the graph, doing nothing if the vertex 
    /// already exists.
    pub fn add_vertex(&mut self, v: Vertex) {
        self.adj_lists.entry(v).or_insert(HashSet::new());
    }

    pub fn add_vertices(&mut self, list: &[Vertex]) {
        for v in list.iter() {
            self.add_vertex(*v);
        }
    }

    pub fn contains_vertex(&self, v: Vertex) -> bool {
        self.adj_lists.contains_key(&v)
    }
    
    /// Attempt to add an edge from vertex u to vertex v.
    /// Return Err if either of the vertices does not exist.
    pub fn add_edge(&mut self, u: Vertex, v: Vertex) -> Result<(), &'static str> {
        if !self.contains_vertex(u) || !self.contains_vertex(v) {
            return Err("one or more input vertices do not exist")
        }
        self.adj_lists.entry(u).and_modify(|l| { l.insert(v); });
        Ok(())
    }

    pub fn add_edges(&mut self, list: &[(Vertex, Vertex)]) -> Result<(), &'static str> {
        for e in list.iter() {
            if !self.contains_vertex(e.0) || !self.contains_vertex(e.1) {
                return Err("one or more input vertices do not exist")
            }
        }
        for e in list.iter() {
            self.adj_lists.entry(e.0).and_modify(|l| { l.insert(e.1); });
        }
        Ok(())
    }

    /// Return a vector of vertices in the graph at the time of calling.
    pub fn vertices(&self) -> Vec<&Vertex> {
        self.adj_lists.keys().collect()
    }

    pub fn edges_from(&self, v: Vertex) -> Result<Vec<&Vertex>, &'static str> {
        let adj_list = self.adj_lists.get(&v).ok_or("vertex does not exist")?;
        Ok(adj_list.iter().collect())
    }

    pub fn num_vertices(&self) -> usize {
        self.adj_lists.len()
    }
}

/// Used in depth-first and breadth-first searches to denote the progress.
#[derive(PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

/// Returns a linear ordering of the graph's vertices using topological sort.
/// Returns an Err if the graph contains a cycle.
/// Based on the algorithm presented by Cormen et al. in chapter 20 of
/// Introduction to Algorithms (4th ed).

pub fn topological_sort(graph: &DirectedGraph) -> Result<LinkedList<Vertex>, &'static str> {
    let mut colors = HashMap::with_capacity(graph.num_vertices());
    for v in graph.vertices().into_iter() {
        colors.insert(*v, Color::White);
    }
    let mut schedule = LinkedList::new();
    for v in graph.vertices().into_iter() {
        if *colors.get(v).unwrap() == Color::White {
            depth_first_visit(&graph, *v, &mut colors, &mut schedule);
        }
    }

    Ok(schedule)
}

fn depth_first_visit(graph: &DirectedGraph, v: Vertex, 
                         colors: &mut HashMap<Vertex, Color>, 
                         schedule: &mut LinkedList<Vertex>)
{
    colors.insert(v, Color::Gray);
    for u in graph.edges_from(v).unwrap().into_iter() {
        if *colors.get(u).unwrap() == Color::White {
            depth_first_visit(graph, *u, colors, schedule);
        }
    }
    schedule.push_front(v);
    colors.insert(v, Color::Black);
}

#[cfg(test)]
mod tests {
    use super::*;
}
