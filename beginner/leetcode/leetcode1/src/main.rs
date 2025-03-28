mod easy;
mod hard;
mod medium;

fn main() {
    let mut g = medium::DFSandBFS::Graph::new(0);
    g.add_vertex('a');
    g.add_vertex('b');
    g.add_vertex('c');
    g.add_vertex('d');
    g.add_vertex('e');
    g.add_vertex('f');
    g.add_vertex('g');
    g.add_vertex('h');

    g.add_edge('a', 'b', true);
    g.add_edge('a', 'c', true);
    g.add_edge('b', 'd', true);
    g.add_edge('b', 'e', true);
    g.add_edge('c', 'f', true);
    g.add_edge('c', 'g', true);
    g.add_edge('e', 'h', true);
    g.add_edge('f', 'h', true);
    g.add_edge('g', 'h', true);

    g.bfs('a');
    println!();
    g.iterative_BFS('a');
}
