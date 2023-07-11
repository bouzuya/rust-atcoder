use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    if n % 2 != 0 {
        println!("-1");
        return;
    }
    let mut count = 0;
    for i in 0..n / 2 {
        if s[i] != s[i + n / 2] {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
