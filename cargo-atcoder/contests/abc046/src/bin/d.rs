use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = (s.len() / 2).saturating_sub(s.iter().copied().filter(|s_i| s_i == &'p').count());
    println!("{}", ans);
}
