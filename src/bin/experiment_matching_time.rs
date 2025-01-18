use std::cmp::max;
use itertools::Itertools;
use log::LevelFilter;
use std::time::Duration;
use AOD4::bipartite::bipartite;
use AOD4::chart::draw_chart;
use AOD4::hopcroft_karp::hopcroft_karp;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .is_test(false)
        .try_init()
        .unwrap();

    let i = 10;
    let iter = i..=10;
    let (matching, time): (Vec<_>, Vec<_>) =
        iter.clone().map(|k| experiment(k, i, 10)).multiunzip();

    // draw_chart(
    //     vec![matching],
    //     vec!["Maximum matching size"],
    //     iter.clone(),
    //     &format!("Maximum matching size in bipartite graph of k = {}", k),
    //     |_, y| y,
    // );

    draw_chart(
        vec![time.iter().map(|x| x.as_secs_f64()).collect()],
        vec!["Algorithm run time per k"],
        iter,
        &format!("Algorithm run time in in bipartite graph of i = {}", i),
        |_, y| y,
    );
}

fn experiment(k: usize, i: usize, repeats: usize) -> (f64, Duration) {
    let i = max(k, i);
    let (matching, time): (Vec<_>, Vec<_>) = (0..repeats)
        .map(|_| {
            let graph = bipartite(k, i);
            hopcroft_karp(&graph, false)
        })
        .multiunzip();
    (
        matching.iter().sum::<usize>() as f64 / matching.len() as f64,
        time.iter().sum::<Duration>() / time.len() as u32,
    )
}
