use proconio::{input, marker::Usize1};

fn adjacency_matrix_edge_index(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let m = uv.len();
    let mut em = vec![vec![m; n]; n];
    for (i, (u, v)) in uv.iter().copied().enumerate() {
        em[u][v] = i;
    }
    em
}

fn dijkstra_with_ps(
    n: usize,
    m: usize,
    inf: i64,
    em: &[Vec<usize>],
    s: usize,
) -> (Vec<i64>, Vec<usize>) {
    let mut path = vec![m; n];
    let mut d_sx = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d_sx[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_su, u))) = pq.pop() {
        if w_su > d_sx[u] {
            continue;
        }
        for v in 0..n {
            if d_sx[v] == inf && em[u][v] != m {
                let w_uv = 1;
                let w_sv = w_su + w_uv;
                d_sx[v] = w_sv;
                path[v] = em[u][v];
                pq.push(std::cmp::Reverse((w_sv, v)));
            }
        }
    }
    (d_sx, path)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(Usize1, Usize1); m],
    };
    let mut em = adjacency_matrix_edge_index(n, &st);

    let inf = n as i64;

    let (d_0x, path) = dijkstra_with_ps(n, m, inf, &em, 0);
    let d_default = d_0x[n - 1];
    if d_default == inf {
        for _ in 0..m {
            println!("-1");
        }
        return;
    }

    let shortest_path = {
        let mut v = n - 1;
        let mut p = vec![];
        while v != 0 {
            let i_e = path[v];
            let (u, v_) = st[i_e];
            assert_eq!(v_, v);
            p.push(i_e);
            v = u;
        }
        p.reverse();
        p
    };

    let mut ans = vec![d_default; m];
    for i_e in shortest_path {
        let (u, v) = st[i_e];
        let backup = em[u][v];
        em[u][v] = m;

        let (d_0x, _) = dijkstra_with_ps(n, m, inf, &em, 0);
        let d = d_0x[n - 1];
        ans[i_e] = if d == inf { -1 } else { d };

        em[u][v] = backup;
    }

    for a in ans {
        println!("{}", a);
    }
}
