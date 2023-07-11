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
        m: usize,
        c: [Chars; n],
    };

    let h = 24;
    let s = n + h;
    let t = s + 1;
    let mut edges = vec![vec![]; n + h + 2];
    for u in 0..n {
        // s -> u
        let i_s = edges[s].len();
        let i_u = edges[u].len();
        edges[s].push((u, 10, i_u));
        edges[u].push((s, 0, i_s));

        // u -> v
        for i in 0..h {
            if c[u][i] != '1' {
                continue;
            }
            let v = n + i;
            let i_u = edges[u].len();
            let i_v = edges[v].len();
            edges[u].push((v, 1, i_v));
            edges[v].push((u, 0, i_u));
        }
    }
    for i in 0..h {
        let v = n + i;
        // v -> t
        let i_v = edges[v].len();
        let i_t = edges[t].len();
        edges[v].push((t, m, i_t));
        edges[t].push((v, 0, i_v));
    }

    let inf = 1_usize << 60;
    let mut sum = 0_usize;
    loop {
        let mut used = vec![false; edges.len()];
        let flow = dfs(&mut used, &mut edges, t, s, inf);
        if flow == 0 {
            break;
        }
        sum += flow;
    }

    let ans = sum >= m * h;
    println!("{}", if ans { "Yes" } else { "No" });
}
