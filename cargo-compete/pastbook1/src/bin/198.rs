use proconio::{input, marker::Usize1};

fn dfs(dp: &mut Vec<usize>, edges: &Vec<Vec<usize>>, u: usize) {
    let mut cs = vec![];
    for v in edges[u].iter().copied() {
        dfs(dp, edges, v);
        cs.push(dp[v]);
    }
    dp[u] = *cs.iter().max().unwrap_or(&0) + *cs.iter().min().unwrap_or(&0) + 1;
}

fn main() {
    input! {
        n: usize,
        b: [Usize1; n - 1]
    }

    let mut edges = vec![vec![]; n];
    for (i, b_i) in b.iter().copied().enumerate() {
        edges[b_i].push(i + 1);
    }

    let mut dp = vec![0_usize; n];
    dfs(&mut dp, &edges, 0);

    let ans = dp[0];
    println!("{}", ans);
}
