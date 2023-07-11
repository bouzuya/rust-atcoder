use proconio::{input, marker::Usize1};

fn dfs(res: &mut Vec<usize>, edges: &[Vec<usize>], u: usize) {
    let mut cs = vec![];
    for v in edges[u].iter().copied() {
        dfs(res, edges, v);
        cs.push(res[v]);
    }
    cs.sort();
    let mut count = 0_usize;
    for c in cs {
        count += c.saturating_sub(count);
        count += 1;
    }
    res[u] = count;
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    };

    let mut edges = vec![vec![]; n];
    for (i, a_i) in a.iter().copied().enumerate() {
        edges[a_i].push(i + 1);
    }

    let mut res = vec![0; n];
    dfs(&mut res, &edges, 0);
    let ans = res[0];
    println!("{}", ans);
}
