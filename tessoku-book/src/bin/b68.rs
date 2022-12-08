use proconio::{input, marker::Usize1};

fn dfs(
    used: &mut Vec<bool>,
    edges: &mut Vec<Vec<(usize, i64, usize)>>,
    g: usize,
    u: usize,
    min: i64,
) -> i64 {
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
        p: [i64; n],
        ab: [(Usize1, Usize1); m],
    };

    let inf = 1_i64 << 60;

    let mut base = 0_i64;
    let s = n;
    let t = n + 1;
    let n = n + 2;
    let mut edges = vec![vec![]; n];
    for (u, p_i) in p.iter().copied().enumerate() {
        let (c_su, c_ut) = if p_i > 0 { (p_i, 0) } else { (0, -p_i) };
        if p_i > 0 {
            base += p_i;
        }
        let (i_s, i_u) = (edges[s].len(), edges[u].len());
        edges[s].push((u, c_su, i_u));
        edges[u].push((s, 0, i_s));
        let (i_u, i_t) = (edges[u].len(), edges[t].len());
        edges[u].push((t, c_ut, i_t));
        edges[t].push((u, 0, i_u));
    }
    for (a, b) in ab {
        let (i_a, i_b) = (edges[a].len(), edges[b].len());
        edges[a].push((b, inf, i_b));
        edges[b].push((a, 0, i_a));
    }

    let mut sum = 0_i64;
    loop {
        let mut used = vec![false; n];
        let flow = dfs(&mut used, &mut edges, t, s, inf);
        if flow == 0 {
            break;
        }
        sum += flow;
    }

    let ans = base - sum;
    println!("{}", ans);
}
