use crate::graph::Graph;
use log::{info, trace};
use rand::{Rng, SeedableRng};
use std::cmp::max;
use std::collections::HashSet;
use std::mem;
// const POWERS: [usize; 17] = [
//     1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
// ];

pub fn hypercube(n: u32) -> Graph {
    let power: usize = 2usize.pow(n);
    let powers: Vec<_> = (0..n).map(|i| 2usize.pow(i)).collect();

    // let mut graph = Graph::with_vertices(power);
    let mut graph = Graph::custom(power, n as usize);

    let mut next = HashSet::new();
    next.insert(0usize);

    let mut rng = rand::rngs::SmallRng::from_os_rng();

    for layer in 0..n {
        trace!("Processing layer {} with {} elements", layer, next.len());

        let current = mem::take(&mut next);

        let l = max(max(layer, n - layer), max(layer + 1, n - (layer + 1)));

        let range = rand::distr::Uniform::new_inclusive(1, 2usize.pow(l)).unwrap();

        for element in current {
            for power in &powers {
                let neighbor = element | power;
                if neighbor != element {
                    graph.add_edge(element, neighbor, rng.sample(range));
                    next.insert(neighbor);
                }
            }
        }
    }
    info!(
        "Done generating {} dimensional hypercube with {} vertices and {} edges",
        n,
        graph.vertices.len(),
        graph.edge_count
    );
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let start = std::time::Instant::now();
        let result = hypercube(1);
        println!("Elapsed time: {:?}", start.elapsed());
        println!("{:?}", result)
    }
}
