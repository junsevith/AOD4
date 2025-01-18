use itertools::Itertools;
use log::LevelFilter;
use std::time::Duration;
use AOD4::chart::draw_chart;
use AOD4::dinic::dinic;
use AOD4::hypercube::hypercube;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .is_test(false)
        .try_init()
        .unwrap();
    let iter = 1..=16;
    let (flow, paths, time): (Vec<_>, Vec<_>, Vec<_>) =
        iter.clone().map(|k| experiment(k, 10)).multiunzip();

    // draw_chart(
    //     vec![flow],
    //     vec!["Maximum flow"],
    //     iter.clone(),
    //     "Dinic: Maximum flow in hypercube of x dimension",
    //     |_, y| y,
    // );
    //
    // draw_chart(
    //     vec![paths],
    //     vec!["Count of expanding paths"],
    //     iter.clone(),
    //     "Dinic: Count of expanding paths in hypercube of x dimension",
    //     |_, y| y,
    // );

    draw_chart(
        vec![time.iter().map(|x| x.as_secs_f64()).collect()],
        vec!["Algorithm run time"],
        iter,
        "Dinic algorithm run time in hypercube of x dimension",
        |_, y| y,
    );
}

fn experiment(k: usize, repeats: usize) -> (f64, f64, Duration) {
    let (flow, paths, time): (Vec<_>, Vec<_>, Vec<_>) = (0..repeats)
        .map(|_| {
            let graph = hypercube(k as u32);
            let len = graph.vertices.len() - 1;
            dinic(graph, 0, len, false)
        })
        .multiunzip();
    (
        flow.iter().sum::<usize>() as f64 / flow.len() as f64,
        paths.iter().sum::<usize>() as f64 / paths.len() as f64,
        time.iter().sum::<Duration>() / time.len() as u32,
    )
}
