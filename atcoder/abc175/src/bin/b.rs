use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l_vec: [i32; n]
    }

    let mut patterns_num = 0;
    for combination in (0..n as usize).combinations(3) {
        let mut edges = vec![
            l_vec[combination[0]],
            l_vec[combination[1]],
            l_vec[combination[2]],
        ];
        edges.sort();

        let is_different = edges[0] != edges[1] && edges[1] != edges[2] && edges[0] != edges[2];
        let is_triangle = edges[0] + edges[1] > edges[2];
        if is_different && is_triangle {
            patterns_num += 1;
        }
    }
    println!("{}", patterns_num);
}
