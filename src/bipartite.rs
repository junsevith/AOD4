use log::info;
use rand::{Rng, SeedableRng};
use crate::graph::Graph;

pub fn bipartite(k: usize, i:usize) -> Graph {
    if i > k {
        panic!("i must be less than or equal to k")
    }
    let power = 2usize.pow(k as u32);
    let mut graph = Graph::custom(2*power, i);
    let mut rng = rand::rngs::SmallRng::from_os_rng();
    let range = rand::distr::Uniform::new_inclusive(power, 2*power-1).unwrap();

    for vertex in 0..power {
        for _ in 0..i {
            let neighbor = rng.sample(range);
            graph.add_edge_undirected(vertex,neighbor,1)
        }
    }

    info!("Bipartite graph generated with {} vertices and {} undirected edges", graph.vertices.len(), graph.edge_count/2);
    graph
}