use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        ab: [(Usize1, Usize1); m],
    };

    let mut edges = vec![vec![]; n];
    for (a_i, b_i) in ab {
        edges[a_i].push(b_i);
        edges[b_i].push(a_i);
    }

    let mut count = 0_usize;
    for (h_i, edges_i) in h.iter().copied().zip(edges.into_iter()) {
        if edges_i.iter().copied().all(|j| h[j] < h_i) {
            count += 1;
        }
    }

    let ans = count;
    println!("{}", ans);
}
