use proconio::{input, marker::Chars};

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
        p: usize,
        q: usize,
        s: [Chars; p],
        ab: [(i64, i64); p],
        cd: [(i64, i64); q],
    }

    let male = 0;
    let female = p;
    let start = p + q;
    let end = p + q + 1;
    let mut edges = vec![vec![]; p + q + 2];
    for (i, ((a, b), s_i)) in ab.iter().copied().zip(s).enumerate() {
        add_edge(&mut edges, start, male + i, 1, 0);
        for (j, s_ij) in s_i.into_iter().enumerate() {
            match s_ij {
                '0' => {}
                '1' => add_edge(&mut edges, male + i, female + j, 1, -a),
                _ => unreachable!(),
            }
        }
        add_edge(&mut edges, male + i, end, 1, -b);
    }
    for (j, (c, d)) in cd.iter().copied().enumerate() {
        add_edge(&mut edges, female + j, end, 1, -c + d);
    }

    let ans = -calc_min_cost_flow(&mut edges, start, end, p as i64)
        + cd.iter().map(|(_, d)| d).sum::<i64>();
    println!("{}", ans);
}
