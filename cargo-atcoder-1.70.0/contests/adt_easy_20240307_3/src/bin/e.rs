use proconio::input;

fn dfs(edges: &[Vec<usize>], depth: &mut Vec<usize>, u: usize, p: usize, l: usize) {
    depth[u] = l;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(edges, depth, v, u, l + 1);
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut edges = vec![vec![]; 4 * n];
    for (i, a_i) in a.iter().copied().enumerate() {
        let i = i + 1;
        edges[a_i].push(2 * i);
        edges[2 * i].push(a_i);
        edges[a_i].push(2 * i + 1);
        edges[2 * i + 1].push(a_i);
    }

    let mut depth = vec![0; 4 * n];
    dfs(&edges, &mut depth, 1, 1, 0);

    for k in 1..=2 * n + 1 {
        println!("{}", depth[k]);
    }
}
