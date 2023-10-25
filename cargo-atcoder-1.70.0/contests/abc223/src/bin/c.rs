use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let sum_a = ab.iter().map(|&(a_i, _)| a_i).sum::<i64>();
    let mut ok = 0_f64;
    let mut ng = 1e18_f64;
    for _ in 0..100 {
        let s = ok + (ng - ok) / 2_f64;
        let mut s_l = 0_f64;
        let mut l = 0_f64;
        for (a_i, b_i) in ab.iter().copied() {
            let s_i = a_i as f64 / b_i as f64;
            if s_l + s_i <= s {
                s_l += s_i;
                l += a_i as f64;
            } else {
                l += (s - s_l) * b_i as f64;
                break;
            }
        }
        let mut s_r = 0_f64;
        let mut r = sum_a as f64;
        for (a_i, b_i) in ab.iter().copied().rev() {
            let s_i = a_i as f64 / b_i as f64;
            if s_r + s_i <= s {
                s_r += s_i;
                r -= a_i as f64;
            } else {
                r -= (s - s_r) * b_i as f64;
                break;
            }
        }
        if l <= r {
            ok = s;
        } else {
            ng = s;
        }
    }
    let s = ng;
    let mut s_l = 0_f64;
    let mut l = 0_f64;
    for (a_i, b_i) in ab.iter().copied() {
        let s_i = a_i as f64 / b_i as f64;
        if s_l + s_i <= s {
            s_l += s_i;
            l += a_i as f64;
        } else {
            l += (s - s_l) * b_i as f64;
            break;
        }
    }
    let ans = l;
    println!("{}", ans);
}
