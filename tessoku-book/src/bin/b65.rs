use proconio::{input, marker::Usize1};

fn dfs(dp: &mut [usize], edges: &[Vec<usize>], u: usize, p: usize) {
    let mut rank = 0_usize;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(dp, edges, v, u);
        rank = rank.max(dp[v] + 1);
    }
    dp[u] = rank;
}

fn main() {
    input! {
        n: usize,
        t: Usize1,
        ab: [(Usize1, Usize1); n - 1],
    };

    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut dp = vec![0; n];
    dfs(&mut dp, &edges, t, t);

    for a in dp {
        println!("{}", a);
    }
}
