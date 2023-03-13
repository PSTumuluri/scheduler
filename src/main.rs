use scheduler::graph::*;

fn main() -> Result<(), &'static str> {
    let mut g = DirectedGraph::new();
    g.add_vertices(&(1..10).collect::<Vec<Vertex>>());
    g.add_edges(&vec![(1, 2), (1, 8), (2, 3), (2, 8), (3, 6), (4, 3), (4, 5), (5, 6), (7, 8)])?;

    let schedule = topological_sort(&g);

    println!("{:?}", schedule);

    Ok(())
}
