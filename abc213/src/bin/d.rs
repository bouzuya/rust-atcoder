use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(res: &mut Vec<usize>, n: usize, e: &[Vec<usize>], u: usize, p: usize) {
    res.push(u);
    for v in e[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(res, n, e, v, u);
        res.push(u);
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let mut e = adjacency_list(n, &ab);
    for u in 0..n {
        e[u].sort();
    }
    let mut ans = vec![];
    dfs(&mut ans, n, &e, 0, n);
    for u in ans {
        println!("{}", u + 1);
    }
}
