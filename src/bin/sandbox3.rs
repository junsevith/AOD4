use log::{debug, info, LevelFilter};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use AOD4::bipartite::bipartite;
use AOD4::dinic::dinic;
use AOD4::hypercube::hypercube;
use AOD4::edmonds_karp::edmonds_karp;
use AOD4::to_jump::jump_max_flow;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Trace).is_test(false).try_init().unwrap();
    debug!("START");

    let start = std::time::Instant::now();
    // let result = bipartite(2, 2);
    let result = hypercube(16);
    info!("Elapsed time: {:?}", start.elapsed());
    debug!("{:?}", result);
    let last = result.vertices.len() - 1;
    dinic(result.clone(), 0, last, false);
    edmonds_karp(result, 0, last, false);

    // let jump = jump_max_flow(result);
    // let file = "model.jl";
    // fs::write(file, jump).expect("Unable to write to a file");
    // info!("Jump code written to {}", file);

    debug!("END");
}
