use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lrv: [(Usize1, Usize1); q]
    };
    let mut c = 0;
    let mut cv = vec![0_usize; n + 1];
    cv[0] = 0;
    for i in 1..n {
        let is_ac = s[i - 1] == 'A' && s[i] == 'C';
        if is_ac {
            c += 1;
        }
        cv[i] = c;
    }
    for (l, r) in lrv {
        let ans = cv[r] - cv[l];
        println!("{}", ans);
    }
}
