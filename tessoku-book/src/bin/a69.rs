use proconio::{input, marker::Chars};

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
        c: [Chars; n],
    };

    let s = 2 * n;
    let t = 2 * n + 1;
    let mut edges = vec![vec![]; 2 * n + 2];
    for (u, c_u) in c.iter().enumerate() {
        for (v, c_uv) in c_u.iter().copied().enumerate() {
            let v = v + n;
            if c_uv == '#' {
                let i_u = edges[u].len();
                let i_v = edges[v].len();
                edges[u].push((v, 1, i_v));
                edges[v].push((u, 0, i_u));
            }
        }
    }

    for u in 0..n {
        let i_s = edges[s].len();
        let i_u = edges[u].len();
        edges[s].push((u, 1, i_u));
        edges[u].push((s, 0, i_s));
        let v = u + n;
        let i_v = edges[v].len();
        let i_t = edges[t].len();
        edges[v].push((t, 1, i_t));
        edges[t].push((v, 0, i_v));
    }

    let inf = 1_usize << 60;
    let mut sum = 0_usize;
    loop {
        let mut used = vec![false; 2 * n + 2];
        let flow = dfs(&mut used, &mut edges, t, s, inf);
        if flow == 0 {
            break;
        }
        sum += flow;
    }

    let ans = sum;
    println!("{}", ans);
}
