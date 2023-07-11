use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let ans = s
        .into_iter()
        .zip(t.into_iter())
        .filter(|(s_i, t_i)| s_i != t_i)
        .count();
    println!("{}", ans);
}
