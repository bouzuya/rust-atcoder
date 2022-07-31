use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m]
    };

    let mut edges = vec![vec![false; n]; n];
    for (u_i, v_i) in uv {
        edges[u_i][v_i] = true;
        edges[v_i][u_i] = true;
    }

    let mut count = 0;
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if edges[a][b] && edges[b][c] && edges[c][a] {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
