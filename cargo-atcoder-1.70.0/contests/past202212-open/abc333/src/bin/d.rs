use proconio::{input, marker::Usize1};

fn dfs(size: &mut Vec<usize>, edges: &Vec<Vec<usize>>, u: usize, p: usize) {
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(size, edges, v, u);
        size[u] += size[v];
    }
    size[u] += 1;
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };

    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }

    if edges[0].len() == 1 {
        println!("1");
        return;
    }

    let mut size = vec![0; n];
    dfs(&mut size, &edges, 0, 0);

    let mut max = 0;
    for v in edges[0].iter().copied() {
        max = max.max(size[v]);
    }

    let ans = n - max;
    println!("{}", ans);
}
