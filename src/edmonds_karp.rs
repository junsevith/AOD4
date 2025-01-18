use crate::graph::{Edge, Graph};
use itertools::Itertools;
use log::{debug, info, trace};
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

pub fn edmonds_karp(mut graph: Graph, start: usize, end: usize, print: bool) -> (usize, usize, Duration) {
    let mut flow = 0;
    let mut iteration = 0;
    let begin = std::time::Instant::now();

    'main: loop {
        // let mut prev: HashMap<usize, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut is_path = false;
        let mut prev = vec![None; graph.vertices.len()];
        queue.push_back(start);

        'bfs: while let Some(vertex) = queue.pop_front() {
            for edge in graph.vertices[vertex].edges.iter() {
                let destination = edge.destination;
                if edge.weight > 0 && prev[destination].is_none() {
                    prev[edge.destination] = Some(vertex);
                    if destination == end {
                        is_path = true;
                        break 'bfs;
                    }
                    queue.push_back(destination);
                }
            }
        }

        if !is_path {
            break 'main;
        }

        let mut path = VecDeque::new();
        path.push_front(end);
        while path.front().unwrap() != &start {
            let vertex = prev[*path.front().unwrap()].unwrap();
            path.push_front(vertex);
        }

        trace!("Path: {:?}", path);

        let bottleneck = path
            .iter()
            .tuple_windows()
            .map(|(a, b)| graph.get_edge(*a, *b).unwrap().weight)
            .min()
            .unwrap();

        flow += bottleneck;

        trace!("Bottleneck: {}", bottleneck);

        path.iter().tuple_windows().for_each(|(a, b)| {
            graph.get_edge_mut(*a, *b).unwrap().weight -= bottleneck;
            match graph.get_edge_mut(*b, *a) {
                None => {
                    graph.add_edge(*b, *a, bottleneck);
                }
                Some(edge) => {
                    edge.weight += bottleneck;
                }
            }
        });

        iteration += 1;

        trace!("Graph after iteration #{}:\n{:?}", iteration, graph);
    }

    let elapsed = begin.elapsed();

    info!("Done calculating max flow");
    println!("Max flow: {}", flow);

    if print {
        println!("Flow per edge (non zero):");
        for vertex in 0..(graph.vertices.len() - 1) {
            for edge in graph.vertices[vertex].edges.iter() {
                let dest = edge.destination;
                if !vertex & dest > 0 {
                    if let Some(back) = graph.get_edge(dest, vertex) {
                        info!("{} -> {} flow: {}", vertex, dest, back.weight);
                    }
                }
            }
        }
    }

    eprintln!("Elapsed time: {:?}", elapsed);
    eprintln!("Expanding path count: {}", iteration);

    (flow, iteration, elapsed)
}

#[derive(Debug)]
pub struct FlowResult {
    flow: usize,
    iterations: usize,
    time: Duration,
}
