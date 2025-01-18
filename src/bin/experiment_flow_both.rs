use itertools::Itertools;
use log::LevelFilter;
use std::time::Duration;
use AOD4::chart::draw_chart;
use AOD4::dinic;
use AOD4::dinic::dinic;
use AOD4::edmonds_karp::edmonds_karp;
use AOD4::hypercube::hypercube;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .is_test(false)
        .try_init()
        .unwrap();
    let iter = 1..=16;
    let (karp, dinic): (Vec<_>, Vec<_>) = iter.clone().map(|k| experiment(k, 10)).multiunzip();

    let (karp_flow, karp_paths, karp_time): (Vec<_>, Vec<_>, Vec<_>) = karp.into_iter().multiunzip();
    let (dinic_flow, dinic_paths, dinic_time): (Vec<_>, Vec<_>, Vec<_>) = dinic.into_iter().multiunzip();

    // draw_chart(
    //     vec![karp_flow, dinic_flow],
    //     vec!["Edmonds Karp", "Dinic"],
    //     iter.clone(),
    //     "Comparison of maximum flow in hypercube of x dimension",
    //     |_, y| y,
    // );
    //
    // draw_chart(
    //     vec![karp_paths, dinic_paths],
    //     vec!["Edmonds Karp", "Dinic"],
    //     iter.clone(),
    //     "Comparison of count of expanding paths in hypercube of x dimension",
    //     |_, y| y,
    // );

    draw_chart(
        vec![karp_time.iter().map(|x| x.as_secs_f64()).collect(), dinic_time.iter().map(|x| x.as_secs_f64()).collect()],
        vec!["Edmonds Karp", "Dinic"],
        iter,
        "Comparison of algorithm run time in hypercube of x dimension in seconds (log2 scale)",
        |_, y| y.log2(),
    );
}

fn experiment(k: usize, repeats: usize) -> ((f64, f64, Duration), (f64, f64, Duration)) {
    let (karp, dinic): (Vec<_>, Vec<_>) = (0..repeats)
        .map(|_| {
            let graph = hypercube(k as u32);
            let len = graph.vertices.len() - 1;
            let karp = edmonds_karp(graph.clone(), 0, len, false);
            let dinic = dinic(graph, 0, len, false);
            (karp, dinic)
        })
        .unzip();

    let (karp_flow, karp_paths, karp_time): (Vec<_>, Vec<_>, Vec<_>) = karp.into_iter().multiunzip();
    let (dinic_flow, dinic_paths, dinic_time): (Vec<_>, Vec<_>, Vec<_>) = dinic.into_iter().multiunzip();

    (
        (
            average(karp_flow),
            average(karp_paths),
            average_duration(karp_time),
        ),
        (
            average(dinic_flow),
            average(dinic_paths),
            average_duration(dinic_time),
        ),
    )
}

fn average(data: Vec<usize>) -> f64 {
    data.iter().sum::<usize>() as f64 / data.len() as f64
}

fn average_duration(data: Vec<Duration>) -> Duration {
    let sum = data.iter().map(|x| x.as_secs_f64()).sum::<f64>();
    Duration::from_secs_f64(sum / data.len() as f64)
}
