use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(route: &mut Vec<usize>, edges: &[Vec<usize>], y: usize, u: usize, p: usize) {
    if u == y {
        route.push(u);
        return;
    }

    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }

        dfs(route, edges, y, v, u);

        if !route.is_empty() {
            route.push(u);
            return;
        }
    }
}

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
        uv: [(Usize1, Usize1); n - 1],
    };

    let edges = adjacency_list(n, &uv);

    let mut route = vec![];
    dfs(&mut route, &edges, y, x, x);
    route.reverse();

    for a in route {
        println!("{}", a + 1);
    }
}
