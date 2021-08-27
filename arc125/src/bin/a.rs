use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i64; n],
        t: [i64; m],
    };
    let has_s0 = s.iter().any(|&s_i| s_i == 0);
    let has_s1 = s.iter().any(|&s_i| s_i == 1);
    let has_t0 = t.iter().any(|&t_i| t_i == 0);
    let has_t1 = t.iter().any(|&t_i| t_i == 1);
    if (has_t0 && !has_s0) || (has_t1 && !has_s1) {
        println!("-1");
        return;
    }
    if (has_t0 && !has_t1) || (!has_t0 && has_t1) {
        let c = if s[0] == t[0] {
            0
        } else {
            let r = n - s.iter().rposition(|&s_i| s_i != s[0]).unwrap();
            let l = s.iter().position(|&s_i| s_i != s[0]).unwrap();
            cmp::min(l, r)
        };
        let ans = c + m;
        println!("{}", ans);
        return;
    }
    let r = n - s.iter().rposition(|&s_i| s_i != s[0]).unwrap();
    let l = s.iter().position(|&s_i| s_i != s[0]).unwrap();
    let d = cmp::min(l, r);
    let mut first = true;
    let mut c = if s[0] == t[0] {
        0
    } else {
        first = false;
        d
    };
    for (i, t_i) in t.iter().copied().enumerate().skip(1) {
        if t[i - 1] != t_i {
            if first {
                first = false;
                c += d;
            } else {
                c += 1;
            }
        }
    }
    let ans = c + m;
    println!("{}", ans);
}
