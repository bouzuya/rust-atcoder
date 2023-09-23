use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let mut ok = true;
    let mut p = n[0];
    for i in 1..n.len() {
        if p <= n[i] {
            ok = false;
            break;
        }
        p = n[i];
    }
    let ans = ok;
    println!("{}", if ans { "Yes" } else { "No" });
}
