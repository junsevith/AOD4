pub mod graph;
pub mod hypercube;
pub mod edmonds_karp;
pub mod to_jump;
pub mod hopcroft_karp;
pub mod bipartite;
pub mod chart;
pub mod dinic;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
