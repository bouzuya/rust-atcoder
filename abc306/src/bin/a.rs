use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut t = String::new();
    for s_i in s {
        t.push(s_i);
        t.push(s_i);
    }
    let ans = t;
    println!("{}", ans);
}
