use proconio::{input, marker::Usize1};

fn dfs(max: &mut Vec<usize>, edges: &[Vec<usize>], u: usize, p: usize) {
    max[u] = max[u].max(max[p].saturating_sub(1));
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(max, edges, v, u);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; n - 1],
        xy: [(Usize1, usize); m],
    };

    let mut edges = vec![vec![]; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        edges[i + 1].push(p_i);
        edges[p_i].push(i + 1);
    }

    let mut max = vec![0; n];
    for (x, y) in xy {
        max[x] = max[x].max(y + 1);
    }

    dfs(&mut max, &edges, 0, 0);

    let ans = max.iter().filter(|&&c| c > 0).count();
    println!("{}", ans);
}
