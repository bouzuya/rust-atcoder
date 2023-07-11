use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;

fn main() {
    input! {
        s: Chars,
    };
    let set = s.into_iter().collect::<BTreeSet<char>>();
    let ans = set.iter().chain(set.iter()).collect::<String>();
    println!("{}", ans);
}
