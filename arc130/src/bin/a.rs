use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut ans = 0_usize;
    let mut count = 1_usize;
    let mut p = s[0];
    for s_i in s.into_iter().skip(1) {
        if s_i == p {
            count += 1;
        } else {
            ans += count * (count - 1) / 2;
            count = 1;
        }
        p = s_i;
    }
    ans += count * (count - 1) / 2;
    println!("{}", ans);
}
