use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let set = s
        .iter()
        .map(|&c| c)
        .collect::<std::collections::BTreeSet<char>>();
    let ans = if s.len() == set.len() { "yes" } else { "no" };
    println!("{}", ans);
}
