use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        mut s: Chars,
        n: usize,
        lr: [(Usize1, Usize1); n],
    };
    for (l_i, r_i) in lr {
        let mut l = l_i;
        let mut r = r_i;
        while l < r {
            let t = s[l];
            s[l] = s[r];
            s[r] = t;
            l += 1;
            r -= 1;
        }
    }
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
