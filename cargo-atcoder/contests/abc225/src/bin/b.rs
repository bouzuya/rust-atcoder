use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let e = adjacency_list(n, &ab);
    for u in 0..n {
        if e[u].len() == n - 1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
