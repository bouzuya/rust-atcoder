use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let ans = s
        .iter()
        .zip(t.iter())
        .map(|(s_i, t_i)| s_i == t_i)
        .filter(|&b| b)
        .count();
    println!("{}", ans);
}
