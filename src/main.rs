use std::collections::{HashSet, HashMap};
use scheduler::graph::*;

fn main() {
    let mut g = DirectedGraph::new();
    let adj_list1 = HashSet::from([2, 8]);
    let adj_list2 = HashSet::from([3, 8]);
    let adj_list3 = HashSet::from([6]);
    let adj_list4 = HashSet::from([3, 5]);
    let adj_list5 = HashSet::from([6]);
    let adj_list6 = HashSet::new();
    let adj_list7 = HashSet::from([8]);
    let adj_list8 = HashSet::new();
    let adj_list9 = HashSet::new();

    g.insert(1, adj_list1);
    g.insert(2, adj_list2);
    g.insert(3, adj_list3);
    g.insert(4, adj_list4);
    g.insert(5, adj_list5);
    g.insert(6, adj_list6);
    g.insert(7, adj_list7);
    g.insert(8, adj_list8);
    g.insert(9, adj_list9);

    let schedule = topological_sort(&g);

    println!("{:?}", schedule);
}
