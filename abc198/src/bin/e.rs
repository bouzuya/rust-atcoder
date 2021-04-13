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

fn dfs(e: &Vec<Vec<usize>>, c: &Vec<usize>, color: &mut Vec<usize>, good: &mut Vec<bool>, u: usize, p: usize) {
    if color[c[u]] > 0 {
        good[u] = false;
    }
    color[c[u]] += 1;
    for &v in e[u].iter() {
        if v != p {
            dfs(e, c, color, good, v, u);
        }
    }
    color[c[u]] -= 1;
}

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        ab: [(Usize1, Usize1); n - 1],
    };

    let e = adjacency_list(n, &ab);
    let mut good = vec![true; n];
    let max_c = 100_000_usize;
    let mut color = vec![0; max_c];
    dfs(&e, &c, &mut color, &mut good, 0, 0);
    for (i, &is_good) in good.iter().enumerate() {
        if is_good {
            println!("{}", i + 1);
        }
    }
}
