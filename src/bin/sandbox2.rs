use std::fs;
use log::{debug, info, LevelFilter};
use AOD4::bipartite::bipartite;
use AOD4::hypercube::hypercube;
use AOD4::hopcroft_karp::hopcroft_karp;
use AOD4::edmonds_karp::edmonds_karp;
use AOD4::to_jump::{jump_matching, jump_max_flow};

fn main() {
    env_logger::builder().filter_level(LevelFilter::Trace).is_test(false).try_init().unwrap();
    debug!("START");

    let start = std::time::Instant::now();
    let result = bipartite(3, 2);
    info!("Elapsed time: {:?}", start.elapsed());
    debug!("{:?}", result);

    let res = hopcroft_karp(&result, true);

    let jump = jump_matching(result);
    let file = "model2.jl";
    fs::write(file, jump).expect("Unable to write to a file");
    info!("Jump code written to {}", file);

    debug!("END");
}