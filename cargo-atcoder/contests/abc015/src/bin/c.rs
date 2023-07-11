use proconio::input;

fn dfs(n: usize, k: usize, t: &[Vec<usize>], x: usize, i: usize) -> bool {
    if i == n {
        x == 0
    } else {
        (0..k).any(|j| dfs(n, k, &t, x ^ t[i][j], i + 1))
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; k]; n],
    };
    let found = dfs(n, k, &t, 0, 0);
    println!("{}", if found { "Found" } else { "Nothing" });
}
