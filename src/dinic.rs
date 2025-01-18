use crate::graph::{Edge, Graph};
use itertools::Itertools;
use log::{debug, info, trace};
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

pub fn dinic(mut graph: Graph, start: usize, end: usize, print: bool) -> (usize, usize, Duration) {
    let mut flow = 0;
    let mut paths = 0;
    let begin = std::time::Instant::now();

    'main: loop {
        // let mut prev: HashMap<usize, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut level_graph = Graph::with_vertices(graph.vertices.len());
        let mut is_path = false;
        let mut level = vec![None; graph.vertices.len()];
        queue.push_back(start);
        level[start] = Some(0);

        'bfs: while let Some(vertex) = queue.pop_front() {
            for edge in graph.vertices[vertex].edges.iter() {
                let dest = edge.destination;
                if edge.weight > 0 {
                    if dest == end {
                        is_path = true;
                    }
                    if level[dest].is_none() {
                        level[dest] = Some(level[vertex].unwrap() + 1);
                        level_graph.add_edge(vertex, dest, edge.weight);
                        queue.push_back(dest);
                    } else if level[dest].unwrap() == level[vertex].unwrap() + 1 {
                        level_graph.add_edge(vertex, dest, edge.weight);
                    }
                }
            }
        }

        if !is_path {
            break 'main;
        }

        trace!("Found level graph: {:?}", level_graph);

        'blocking_flows: loop {
            let mut path = vec![start];
            let mut last = start;
            'dfs: while last != end {
                if level_graph.vertices[last].edges.is_empty() {
                    if last == start {
                        break 'blocking_flows;
                    } else {
                        path.pop();
                        last = *path.last().unwrap();
                        level_graph.vertices[last].edges.pop();
                    }
                } else {
                    let edge = level_graph.vertices[last].edges.last().unwrap();
                    path.push(edge.destination);
                    last = edge.destination;
                }
            }

            trace!("Found ath: {:?}", path);

            let bottleneck = path
                .iter()
                .tuple_windows()
                .map(|(a, b)| graph.get_edge(*a, *b).unwrap().weight)
                .min()
                .unwrap();

            flow += bottleneck;

            path.iter().tuple_windows().for_each(|(a, b)| {
                {
                    let edge  = graph.get_edge_mut(*a, *b).unwrap();
                    edge.weight -= bottleneck;
                    if edge.weight == 0 {
                        level_graph.remove_edge(*a, *b);
                    }
                }
                match graph.get_edge_mut(*b, *a) {
                    None => {
                        graph.add_edge(*b, *a, bottleneck);
                    }
                    Some(edge) => {
                        edge.weight += bottleneck;
                    }
                }
            });
            paths += 1;
        }
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
    eprintln!("Expanding path count: {}", paths);

    (flow, paths, elapsed)
}

#[derive(Debug)]
pub struct FlowResult {
    flow: usize,
    iterations: usize,
    time: Duration,
}
