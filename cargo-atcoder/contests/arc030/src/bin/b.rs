use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(e: &Vec<Vec<usize>>, h: &Vec<u8>, u: usize, p: usize) -> (bool, usize) {
    let mut res = (h[u] == 1, 0);
    for &v in e[u].iter() {
        if v == p {
            continue;
        }
        let (has, cost) = dfs(e, h, v, u);
        res.0 |= has;
        res.1 += if has { cost + 2 } else { 0 };
    }
    res
}

fn main() {
    input! {
        n: usize,
        x: Usize1,
        h: [u8; n],
        ab: [(Usize1, Usize1); n - 1],
    };

    let e = adjacency_list(n, &ab);

    let (has, cost) = dfs(&e, &h, x, x);
    let ans = if has { cost } else { 0 };
    println!("{}", ans);
}
