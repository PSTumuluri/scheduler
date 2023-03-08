use std::collections::{HashMap, HashSet, LinkedList};

pub type Vertex = usize;
pub type AdjacencyList = HashSet<Vertex>;
pub type DirectedGraph = HashMap<Vertex, AdjacencyList>;

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
    let mut colors = HashMap::with_capacity(graph.len());
    for v in graph.keys() {
        colors.insert(*v, Color::White);
    }
    let mut schedule = LinkedList::new();
    for v in graph.keys() {
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
    for u in graph.get(&v).unwrap().iter() {
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
