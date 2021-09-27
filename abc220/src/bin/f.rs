use std::collections::VecDeque;

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

fn size_of_subtree_dfs(edges: &[Vec<usize>], size: &mut Vec<usize>, u: usize, p: usize) {
    let mut s = 1;
    for v in edges[u].clone() {
        if v == p {
            continue;
        }
        size_of_subtree_dfs(edges, size, v, u);
        s += size[v];
    }
    size[u] = s;
}

fn size_of_subtree(e: &[Vec<usize>], root: usize) -> Vec<usize> {
    let mut res = vec![0; e.len()];
    size_of_subtree_dfs(&e, &mut res, root, root);
    res
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };

    let edges = adjacency_list(n, &uv);
    let sub = size_of_subtree(&edges, 0);
    let mut ans = vec![0; n];
    let mut deque = VecDeque::new();
    deque.push_back((0, 0));
    while let Some((u, p)) = deque.pop_front() {
        if u == 0 {
            ans[0] = depth(&edges, 0).into_iter().sum::<usize>();
        } else {
            ans[u] = ans[p] + n - 2 * sub[u];
        }

        for u_next in edges[u].clone() {
            if u_next == p {
                continue;
            }
            deque.push_back((u_next, u));
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
