use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let mut t = n.clone();
    t.reverse();
    let ans = n == t;
    println!("{}", if ans { "Yes" } else { "No" });
}
