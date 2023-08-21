use proconio::input;

fn add_edge(edges: &mut [Vec<(usize, i64, i64, usize)>], u: usize, v: usize, cap: i64, cost: i64) {
    let index_u = edges[u].len();
    let index_v = edges[v].len();
    edges[u].push((v, cap, cost, index_v));
    edges[v].push((u, 0, -cost, index_u));
}

fn bellman_ford(
    edges: &[Vec<(usize, i64, i64, usize)>],
    s: usize,
) -> (Vec<i64>, Vec<usize>, Vec<usize>) {
    let inf = 1_i64 << 60;
    let mut dist = vec![inf; edges.len()];
    dist[s] = 0;
    let mut prev_vert = vec![0_usize; edges.len()];
    let mut prev_edge = vec![0_usize; edges.len()];
    loop {
        let mut update = false;
        for v in 0..edges.len() {
            if dist[v] == inf {
                continue;
            }
            for (i, (u, cap, cost, _)) in edges[v].iter().copied().enumerate() {
                if cap > 0 && dist[u] > dist[v] + cost {
                    dist[u] = dist[v] + cost;
                    prev_vert[u] = v;
                    prev_edge[u] = i;
                    update = true;
                }
            }
        }
        if !update {
            break;
        }
    }
    (dist, prev_vert, prev_edge)
}

fn calc_min_cost_flow(
    edges: &mut [Vec<(usize, i64, i64, usize)>],
    s: usize,
    t: usize,
    mut capital_f: i64,
) -> i64 {
    let inf = 1_i64 << 60;
    let mut min_cost = 0_i64;
    while capital_f > 0 {
        let (dist, prev_vert, prev_edge) = bellman_ford(edges, s);
        if dist[t] == inf {
            return inf;
        }

        let mut v = t;
        let mut f = capital_f;
        while v != s {
            let u = prev_vert[v];
            let i = prev_edge[v];
            let c = edges[u][i].1;
            f = f.min(c);
            v = u;
        }

        v = t;
        while v != s {
            let u = prev_vert[v];
            let i = prev_edge[v];
            let j = edges[u][i].3;
            edges[u][i].1 -= f;
            edges[v][j].1 += f;
            v = u;
        }

        min_cost += f * dist[t];
        capital_f -= f;
    }
    min_cost
}

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
        b: [i64; n],
        r: [i64; 3],
    }

    let count_v = 3 * (n + 1) + n + 2;
    let rounds = vec![0, n, 2 * n];
    let penalty = rounds[2] + n;
    let starts = vec![penalty + n, penalty + n + 1, penalty + n + 2];
    let start = starts[2] + 1;
    let end = start + 1;

    let mut edges = vec![vec![]; count_v];
    for i in 0..3 {
        add_edge(&mut edges, start, starts[i], m, 0);
    }
    for i in 0..3 {
        for j in 0..n {
            let p_j = a[j] * (b[j].pow((i + 1) as u32)) % r[i];
            add_edge(&mut edges, starts[i], rounds[i] + j, 1, -p_j);
            add_edge(&mut edges, rounds[i] + j, penalty + j, 1, 0);
        }
    }
    for j in 0..n {
        let q_0 = a[j] * b[j];
        let q_1 = a[j] * b[j].pow(2) - q_0;
        let q_2 = a[j] * b[j].pow(3) - q_0 - q_1;

        add_edge(&mut edges, penalty + j, end, 1, q_0);
        add_edge(&mut edges, penalty + j, end, 1, q_1);
        add_edge(&mut edges, penalty + j, end, 1, q_2);
    }

    let ans = -calc_min_cost_flow(&mut edges, start, end, m * 3);
    println!("{}", ans);
}
