use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(e: &Vec<Vec<usize>>, dp: &mut Vec<usize>, u: usize, p: usize) {
    dp[u] = 1;
    for &v in e[u].iter() {
        if v == p {
            continue;
        }
        dfs(e, dp, v, u);
        dp[u] += dp[v];
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let e = adjacency_list(n, &ab);
    let mut dp = vec![0_usize; n];
    dfs(&e, &mut dp, 0, 0);
    let ans = dp.iter().map(|x| x * (n - x)).sum::<usize>();
    println!("{}", ans);
}
