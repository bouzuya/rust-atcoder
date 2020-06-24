use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    let n = s.len();
    if k > n {
        println!("{}", 0);
        return;
    }
    let mut set = std::collections::BTreeSet::new();
    for i in 0..=n - k {
        set.insert(s[i..i + k].iter().collect::<String>());
    }
    let ans = set.len();
    println!("{}", ans);
}
