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
    if s.contains(&0) {
        println!("{}", n);
        return;
    }
    let mut ans = 0;
    for l in 0..n {
        let mut c = 1;
        for r in l..n {
            c *= s[r];
            if c > k {
                break;
            }
            chmax!(ans, r - l + 1);
        }
    }
    println!("{}", ans);
}
