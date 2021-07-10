use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(e: &[Vec<usize>], d: &mut Vec<usize>, u: usize, p: usize, l: usize) {
    d[u] = l;
    for v in e[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(e, d, v, u, l + 1);
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        cd: [(Usize1, Usize1); q],
    };
    let e = adjacency_list(n, &ab);
    let mut d = vec![0; n];
    dfs(&e, &mut d, 0, 0, 0);
    for (c_i, d_i) in cd {
        let l = d[d_i] + d[c_i];
        let road = l % 2 == 1;
        let ans = if road { "Road" } else { "Town" };
        println!("{}", ans);
    }
}
