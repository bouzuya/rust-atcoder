use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        a: Usize1,
        b: Usize1,
    };
    let s = vec![13, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let s_a = s[a];
    let s_b = s[b];
    let ans = if s_a == s_b {
        "Draw"
    } else if s_a > s_b {
        "Alice"
    } else if s_b > s_a {
        "Bob"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
