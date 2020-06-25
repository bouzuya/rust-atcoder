use proconio::input;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        k: i64,
        s: [i64; n],
    };

    // [l, r)
    let mut r = 0;
    let mut p = 1;
    let mut ans = 0;
    for l in 0..n {
        while r < n && p * s[r] <= k {
            p *= s[r];
            r += 1;
        }
        if p == 0 {
            println!("{}", n);
            return;
        }
        chmax!(ans, r - l);
        if l == r {
            r += 1;
        } else {
            p /= s[l];
        }
    }
    println!("{}", ans);
}
