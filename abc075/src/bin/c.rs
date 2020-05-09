use proconio::input;
use proconio::marker::Usize1;

fn dfs(visited: &mut Vec<bool>, e: &Vec<Vec<usize>>, ab: (usize, usize), u: usize) {
    if visited[u] {
        return;
    }
    visited[u] = true;
    for &v in e[u].iter() {
        if ab == (u, v) || ab == (v, u) {
            continue;
        }
        dfs(visited, e, ab, v);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut e = vec![vec![]; n];
    for &(a_i, b_i) in ab.iter() {
        e[a_i].push(b_i);
        e[b_i].push(a_i);
    }
    let mut count = 0;
    for &ab_i in ab.iter() {
        let mut visited = vec![false; n];
        dfs(&mut visited, &e, ab_i, 0);
        if !visited.iter().all(|&b| b) {
            count += 1;
        }
    }
    println!("{}", count);
}
