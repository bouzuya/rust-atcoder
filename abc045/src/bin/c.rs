use proconio::input;
use proconio::marker::Chars;

fn dfs(d: &Vec<u64>, i: usize, s: u64, c: u64) -> u64 {
    if i < d.len() {
        dfs(d, i + 1, s + c, d[i]) + dfs(d, i + 1, s, c * 10 + d[i])
    } else {
        s + c
    }
}

fn main() {
    input! {
        s: Chars
    };
    let d = s
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();
    let ans = dfs(&d, 1, 0, d[0]);
    println!("{}", ans);
}
