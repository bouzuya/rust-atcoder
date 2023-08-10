use proconio::{input, marker::Usize1};

fn dfs(
    depth: &mut Vec<usize>,
    parent: &mut Vec<usize>,
    edges: &[Vec<usize>],
    u: usize,
    p: usize,
    d: usize,
) {
    depth[u] = d;
    parent[u] = p;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(depth, parent, edges, v, u, d + 1);
    }
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
    let (mut u, mut v) = if depth[u] < depth[v] { (u, v) } else { (v, u) };
    for k in 0..k_len {
        if (((depth[v] - depth[u]) >> k) & 1) == 1 {
            v = parent[k][v];
        }
    }
    if u == v {
        return u;
    }
    for k in (0..k_len).rev() {
        if parent[k][u] != parent[k][v] {
            u = parent[k][u];
            v = parent[k][v];
        }
    }
    assert_eq!(parent[0][u], parent[0][v]);
    parent[0][u]
}

fn get_leader(leader: &mut [usize], u: usize) -> usize {
    if leader[u] == u {
        u
    } else {
        leader[u] = get_leader(leader, leader[u]);
        leader[u]
    }
}

fn merge(leader: &mut [usize], depth: &[usize], u: usize, v: usize) {
    let (mut x, mut y) = (get_leader(leader, u), get_leader(leader, v));
    if x == y {
        return;
    }
    if depth[x] > depth[y] {
        std::mem::swap(&mut x, &mut y);
    }
    leader[y] = leader[x];
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        uvc: [(Usize1, Usize1, usize); q],
    }

    let mut edges = vec![vec![]; n];
    for (a, b) in ab.iter().copied() {
        edges[a].push(b);
        edges[b].push(a);
    }

    let (depth, parent1) = {
        let mut depth = vec![0; n];
        let mut parent1 = vec![0; n];
        dfs(&mut depth, &mut parent1, &edges, 0, 0, 0);
        (depth, parent1)
    };

    let mut index = vec![n; n];
    for (i, (a, b)) in ab.iter().copied().enumerate() {
        let child = if depth[a] < depth[b] { b } else { a };
        index[child] = i;
    }

    let parent = parent_doubling(&parent1);

    let mut color = vec![0; n];
    let mut leader = (0..n).collect::<Vec<usize>>();
    for (mut u, mut v, c) in uvc.into_iter().rev() {
        let lca = lca_by_doubling(&depth, &parent, u, v);

        // u -> lca
        u = get_leader(&mut leader, u);
        while depth[u] > depth[lca] {
            color[index[u]] = c;
            let p = parent1[u];
            merge(&mut leader, &depth, u, p);
            u = get_leader(&mut leader, p);
        }

        // v -> lca
        v = get_leader(&mut leader, v);
        while depth[v] > depth[lca] {
            color[index[v]] = c;
            let p = parent1[v];
            merge(&mut leader, &depth, v, p);
            v = get_leader(&mut leader, p);
        }
    }

    for i in 0..n - 1 {
        println!("{}", color[i]);
    }
}
