use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    };
    let mut prev = s[0];
    let mut eq = 0_usize;
    let mut ne = 0_usize;
    for s_i in s.into_iter().skip(1) {
        if prev == s_i {
            eq += 1;
        } else {
            ne += 1;
        }
        prev = s_i;
    }
    let ans = eq + ne.min(2 * k);
    println!("{}", ans);
}
