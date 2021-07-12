use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn depth_dfs(edges: &[Vec<usize>], depth: &mut Vec<usize>, u: usize, p: usize, l: usize) {
    depth[u] = l;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        depth_dfs(edges, depth, v, u, l + 1);
    }
}

fn depth(e: &[Vec<usize>], root: usize) -> Vec<usize> {
    let mut res = vec![0; e.len()];
    depth_dfs(&e, &mut res, root, root, 0);
    res
}

fn parent_dfs(edges: &[Vec<usize>], parent: &mut Vec<usize>, u: usize, p: usize) {
    parent[u] = p;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        parent_dfs(edges, parent, v, u);
    }
}

fn parent(e: &[Vec<usize>], root: usize) -> Vec<usize> {
    let mut res = vec![e.len(); e.len()];
    parent_dfs(&e, &mut res, root, e.len());
    res
}

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

fn dist(d: &[usize], p: &[Vec<usize>], u: usize, v: usize) -> usize {
    let lca = lca_by_doubling(&d, &p, u, v);
    d[u] + d[v] - d[lca] * 2
}

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n - 1],
        q: usize,
        ab: [(Usize1, Usize1); q],
    };

    let e = adjacency_list(n, &xy);
    let d = depth(&e, 0);
    let p = parent(&e, 0);
    let p = parent_doubling(&p);
    for (a_i, b_i) in ab {
        let l = dist(&d, &p, a_i, b_i) + 1;
        println!("{}", l);
    }
}
