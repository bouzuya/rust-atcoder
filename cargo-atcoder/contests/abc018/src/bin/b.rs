use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        mut s: Chars,
        n: usize,
        lr: [(Usize1, Usize1); n],
    };
    for (l_i, r_i) in lr {
        let len = r_i + 1 - l_i;
        for j in 0..len / 2 {
            let (l, r) = (l_i + j, l_i + len - 1 - j);
            let t = s[l];
            s[l] = s[r];
            s[r] = t;
        }
    }
    println!("{}", s.iter().collect::<String>());
}
