use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let mut c = 0_i64;
    for (i, &s_i) in s.iter().enumerate().skip(1) {
        if s_i != s[i - 1] {
            c += 1;
        }
    }
    for _ in 0..k {
        c -= 2;
    }
    let ans = n as i64 - 1 - std::cmp::max(c, 0);
    println!("{}", ans);
}
