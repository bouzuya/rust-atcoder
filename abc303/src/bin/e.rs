use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };

    let edges = adjacency_list(n, &uv);

    // 0: root, 1: leaf, 2: unknown
    let mut root_or_leaf = vec![2; n];
    let mut roots = VecDeque::new();
    for u in 0..n {
        if edges[u].len() == 1 {
            root_or_leaf[u] = 1;
            let p = edges[u][0];
            root_or_leaf[p] = 0;
            roots.push_back(p);
        }
    }

    let mut used = vec![false; n];
    while let Some(root) = roots.pop_front() {
        if used[root] {
            continue;
        }
        used[root] = true;
        for leaf in edges[root].iter().copied() {
            if root_or_leaf[leaf] != 2 {
                continue;
            }
            root_or_leaf[leaf] = 1;
            if edges[leaf].len() > 1 {
                for v in edges[leaf].iter().copied() {
                    if v == root {
                        continue;
                    }
                    if root_or_leaf[v] != 2 {
                        continue;
                    }
                    root_or_leaf[v] = 1;
                    for w in edges[v].iter().copied() {
                        if w == leaf {
                            continue;
                        }
                        if root_or_leaf[w] != 2 {
                            continue;
                        }
                        if used[w] {
                            continue;
                        }
                        root_or_leaf[w] = 0;
                        roots.push_back(w);
                    }
                }
            }
        }
    }

    let mut ans = vec![];
    for u in 0..n {
        if root_or_leaf[u] == 0 {
            ans.push(edges[u].len());
        }
    }
    ans.sort();

    for a in ans {
        println!("{}", a);
    }
}
