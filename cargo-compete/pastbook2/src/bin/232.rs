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

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n - 1],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }

    let (depth, parent) = {
        let edges = {
            let mut edges = vec![vec![]; n];
            for (x, y) in xy.iter().copied() {
                edges[x].push(y);
                edges[y].push(x);
            }
            edges
        };

        let (depth, parent_0) = {
            let mut depth = vec![n + 1; n];
            let mut parent = vec![n + 1; n];
            dfs(&mut depth, &mut parent, &edges, 0, 0, 0);
            (depth, parent)
        };

        let parent = {
            let mut parent = vec![vec![n; n]; n.next_power_of_two().trailing_zeros() as usize + 1];
            parent[0] = parent_0;
            for k in 0..parent.len() - 1 {
                for i in 0..n {
                    if parent[k][i] == n {
                        parent[k + 1][i] = n;
                    } else {
                        parent[k + 1][i] = parent[k][parent[k][i]];
                    }
                }
            }
            parent
        };

        (depth, parent)
    };

    let lca_by_doubling = |a: usize, b: usize| -> usize {
        let (mut a, mut b) = if depth[a] < depth[b] { (a, b) } else { (b, a) };
        for (k, parent_k) in parent.iter().enumerate() {
            if (((depth[b] - depth[a]) >> k) & 1) == 1 {
                b = parent_k[b];
            }
        }
        if a == b {
            return a;
        }
        for k in (0..parent.len()).rev() {
            if parent[k][a] != parent[k][b] {
                a = parent[k][a];
                b = parent[k][b];
            }
        }
        assert_eq!(parent[0][a], parent[0][b]);
        parent[0][a]
    };

    for (a, b) in ab {
        let lca = lca_by_doubling(a, b);
        let ans = depth[a] - depth[lca] + depth[b] - depth[lca] + 1;
        println!("{}", ans);
    }
}
