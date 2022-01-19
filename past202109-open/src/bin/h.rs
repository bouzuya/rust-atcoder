use proconio::{input, marker::Usize1};

fn parent_doubling(p: &[usize]) -> Vec<Vec<usize>> {
    let n = p.len();
    let k_len = p.len().next_power_of_two().trailing_zeros() as usize;
    let mut res = vec![vec![n; n]; k_len];
    for (i, p_i) in p.iter().copied().enumerate() {
        res[0][i] = p_i;
    }
    for k in 0..k_len - 1 {
        for i in 0..p.len() {
            if res[k][i] == n {
                res[k + 1][i] = n;
            } else {
                res[k + 1][i] = res[k][res[k][i]];
            }
        }
    }
    res
}

fn lca_by_doubling(depth: &[usize], parent: &[Vec<usize>], u: usize, v: usize) -> usize {
    let k_len = parent.len();
    // u と v の深さを揃える
    let (mut u, mut v) = if depth[u] > depth[v] { (v, u) } else { (u, v) };
    for k in 0..k_len {
        if (((depth[v] - depth[u]) >> k) & 1) == 1 {
            v = parent[k][v];
        }
    }
    if u == v {
        return u;
    }
    // 二分探索する
    for k in (0..k_len).rev() {
        if parent[k][u] != parent[k][v] {
            u = parent[k][u];
            v = parent[k][v];
        }
    }
    assert_eq!(parent[0][u], parent[0][v]);
    parent[0][u]
}

fn depth_dfs(edges: &[Vec<(usize, usize)>], depth: &mut Vec<usize>, u: usize, p: usize, l: usize) {
    depth[u] = l;
    for (v, _) in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        depth_dfs(edges, depth, v, u, l + 1);
    }
}

fn depth(e: &[Vec<(usize, usize)>], root: usize) -> Vec<usize> {
    let mut res = vec![0; e.len()];
    depth_dfs(&e, &mut res, root, root, 0);
    res
}

fn dist_dfs(edges: &[Vec<(usize, usize)>], dist: &mut Vec<usize>, u: usize, p: usize, l: usize) {
    dist[u] = l;
    for (v, w) in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dist_dfs(edges, dist, v, u, l + w);
    }
}

fn dist(e: &[Vec<(usize, usize)>], root: usize) -> Vec<usize> {
    let mut res = vec![0; e.len()];
    dist_dfs(&e, &mut res, root, root, 0);
    res
}

fn parent_dfs(edges: &[Vec<(usize, usize)>], parent: &mut Vec<usize>, u: usize, p: usize) {
    parent[u] = p;
    for (v, _) in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        parent_dfs(edges, parent, v, u);
    }
}

fn parent(e: &[Vec<(usize, usize)>], root: usize) -> Vec<usize> {
    let mut res = vec![e.len(); e.len()];
    parent_dfs(&e, &mut res, root, e.len());
    res
}

fn adjacency_list(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn main() {
    input! {
        n: usize,
        x: usize,
        abc: [(Usize1, Usize1, usize); n - 1],
    };

    let e = adjacency_list(n, &abc);
    let p = parent(&e, 0);
    let d = depth(&e, 0);
    let dist = dist(&e, 0);
    let pd = parent_doubling(&p);
    for u in 0..n {
        for v in u + 1..n {
            let lca = lca_by_doubling(&d, &pd, u, v);
            let l = dist[u] + dist[v] - dist[lca] * 2;
            if l == x {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
