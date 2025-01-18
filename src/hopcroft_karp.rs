use crate::graph::Graph;
use log::{info, trace};
use std::collections::VecDeque;
use std::time::Duration;

pub fn hopcroft_karp(graph: &Graph, print: bool) -> (usize, Duration) {
    let len = graph.vertices.len();
    let mut matching = vec![None; len];
    let mut matching_size = 0;
    let begin = std::time::Instant::now();

    for vertex in 0..len {
        if matching[vertex].is_none() {
            trace!("Starting from vertex {}", vertex);
            let mut prev = vec![None; len];
            let mut queue = VecDeque::new();
            let mut last = None;
            queue.push_back(vertex);

            'bfs: while let Some(vertex) = queue.pop_front() {
                for edge in graph.vertices[vertex].edges.iter() {
                    let destination = edge.destination;
                    if prev[destination].is_none() {
                        prev[destination] = Some(vertex);
                        match matching[destination] {
                            None => {
                                last = Some(destination);
                                break 'bfs;
                            }
                            Some(matched) => {
                                prev[matched] = Some(destination);
                                queue.push_back(matched);
                            }
                        }
                    }
                }
            }

            if last.is_some() {
                matching_size += 1;
            }

            while let Some(current) = last {
                let next = prev[current].unwrap();
                matching[current] = Some(next);
                matching[next] = Some(current);
                last = prev[next];
            }
        }
    }

    let elapsed = begin.elapsed();

    info!("Done calculating maximum matching");
    println!("Maximum matching size: {}", matching_size);
    println!("Elapsed time: {:?}", elapsed);
    if print {
        println!("Matching found:");
        for i in 0..(len / 2) {
            println!("{} -> {:?}", i, matching[i]);
        }
    }

    (matching_size, elapsed)
}

pub struct MatchingResult {
    pub matching_size: usize,
    pub time: Duration,
}
