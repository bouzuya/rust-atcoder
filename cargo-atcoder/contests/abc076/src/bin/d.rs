// https://atcoder.jp/contests/abc076/submissions/9351616
use proconio::input;

macro_rules! chmin {
    ($e: expr, $v: expr) => {
        $e = if $e < $v { $e } else { $v };
    };
}

fn main() {
    input! {
        n: usize,
        t: [usize; n],
        v: [usize; n],
    };
    let sum_t = t.iter().sum::<usize>();
    let mut l = 0;
    let mut max_v = vec![sum_t; sum_t + 1];
    for (&t_i, &v_i) in t.iter().zip(v.iter()) {
        let r = l + t_i;
        for x in l..=r {
            chmin!(max_v[x], v_i);
        }
        l = r;
    }
    max_v[0] = 0;
    for i in 0..sum_t {
        chmin!(max_v[i + 1], max_v[i] + 1);
    }
    max_v[sum_t] = 0;
    for i in (0..sum_t).rev() {
        chmin!(max_v[i], max_v[i + 1] + 1);
    }

    let mut ans = 0_f64;
    let mut l = 0;
    for (&t_i, &v_i) in t.iter().zip(v.iter()) {
        let r = l + t_i;
        for x in l..r {
            let v_l = max_v[x] as f64;
            let v_r = max_v[x + 1] as f64;
            ans += if v_l == v_r && v_l < v_i as f64 {
                v_r + 0.25_f64
            } else {
                (v_l + v_r) * 0.5_f64
            };
        }
        l = r;
    }
    println!("{}", ans);
}
