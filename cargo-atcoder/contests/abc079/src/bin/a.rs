use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let ans = (n[0] == n[1] && n[1] == n[2]) || (n[1] == n[2] && n[2] == n[3]);
    println!("{}", if ans { "Yes" } else { "No" });
}
