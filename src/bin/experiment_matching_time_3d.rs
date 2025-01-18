use itertools::Itertools;
use log::LevelFilter;
use plotters::prelude::{BitMapBackend, ChartBuilder, Color, Cubiod, IntoDrawingArea, BLACK, BLUE, WHITE};
use std::cmp::min;
use std::time::Duration;
use AOD4::bipartite::bipartite;
use AOD4::hopcroft_karp::hopcroft_karp;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .is_test(false)
        .try_init()
        .unwrap();

    let root = BitMapBackend::new("charts/3d_time.png", (1280, 720)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Algorithm run time in microseconds per k and i", ("sans-serif", 40))
        .build_cartesian_3d(3usize..10, 0usize..250, 1usize..10)
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.pitch = 0.3;
        pb.yaw = 3.95;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart.configure_axes().draw().unwrap();
    chart.draw_series(
        (3..10).rev()
            .map(|x| std::iter::repeat(x).zip((1..10).rev()))
            .flatten()
            .map(|(x,z)| {
                // let (paths, time) = experiment(x, z, 10);
                Cubiod::new([(x, 0, z), (x + 1, experiment(x, z, 10).1.as_micros() as usize, z + 1)], BLUE.filled(), &BLACK)
            })
    ).unwrap();
}

fn experiment(k: usize, i: usize, repeats: usize) -> (f64, Duration) {
    let i = min(k, i);
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
