use proconio::{input, marker::Usize1};

fn dfs(
    used: &mut Vec<bool>,
    route: &mut Vec<usize>,
    edges: &[Vec<usize>],
    u: usize,
    p: usize,
) -> bool {
    if u == edges.len() - 1 {
        return true;
    }
    if used[u] {
        return false;
    }
    used[u] = true;

    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        if dfs(used, route, edges, v, u) {
            route.push(v);
            return true;
        }
    }

    false
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut used = vec![false; n];
    let mut route = vec![];
    if dfs(&mut used, &mut route, &edges, 0, 0) {
        route.push(0);
    }

    for a in route.into_iter().rev() {
        println!("{}", a + 1);
    }
}
