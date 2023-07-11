use proconio::{input, marker::Usize1};

fn dfs(dp: &mut Vec<usize>, edges: &[Vec<usize>], u: usize, p: usize) {
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(dp, edges, v, u);
        dp[u] += dp[v] + 1;
    }
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    };
    let mut edges = vec![vec![]; n];
    for (i, a_i) in a.iter().copied().enumerate() {
        edges[i + 1].push(a_i);
        edges[a_i].push(i + 1);
    }

    let mut dp = vec![0; n];
    dfs(&mut dp, &edges, 0, 0);

    for a in dp {
        println!("{}", a);
    }
}
