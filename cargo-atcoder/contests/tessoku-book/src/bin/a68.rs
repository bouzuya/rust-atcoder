use proconio::{input, marker::Usize1};

fn dfs(
    used: &mut Vec<bool>,
    edges: &mut Vec<Vec<(usize, usize, usize)>>,
    g: usize,
    u: usize,
    min: usize,
) -> usize {
    if g == u {
        return min;
    }
    used[u] = true;

    let len = edges[u].len();
    for i_u in 0..len {
        let (v, cap, i_v) = edges[u][i_u];

        if used[v] {
            continue;
        }
        if cap == 0 {
            continue;
        }

        let flow = dfs(used, edges, g, v, min.min(cap));
        if flow > 0 {
            edges[u][i_u].1 -= flow;
            edges[v][i_v].1 += flow;
            return flow;
        }
    }

    0
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    };

    let mut edges = vec![vec![]; n];
    for (a, b, c) in abc.iter().copied() {
        let i_a = edges[a].len();
        let i_b = edges[b].len();
        edges[a].push((b, c, i_b));
        edges[b].push((a, 0, i_a));
    }

    let inf = 1_usize << 60;
    let mut sum = 0_usize;
    loop {
        let mut used = vec![false; n];
        used[0] = true;
        let flow = dfs(&mut used, &mut edges, n - 1, 0, inf);
        if flow == 0 {
            break;
        }
        sum += flow;
    }

    let ans = sum;
    println!("{}", ans);
}
