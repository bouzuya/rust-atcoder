use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(n: usize, e: &[Vec<usize>], used: &mut [bool], u: usize, p: usize) -> bool {
    if used[u] {
        return false;
    }
    used[u] = true;

    if e[u].len() > 2 {
        return false;
    }
    for v in e[u].iter().copied() {
        if v == p {
            continue;
        }

        if !dfs(n, e, used, v, u) {
            return false;
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let e = adjacency_list(n, &ab);
    let mut used = vec![false; n];
    let mut ans = true;
    for u in 0..n {
        if !used[u] {
            if !dfs(n, &e, &mut used, u, u) {
                ans = false;
                break;
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
