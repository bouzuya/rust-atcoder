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
    let c = std::iter::once(1_i64)
        .chain(s.iter().scan(1_i64, |acc, &s_i| {
            *acc *= s_i;
            Some(*acc)
        }))
        .collect::<Vec<_>>();
    let mut ans = 0;
    for l in 0..=n {
        for r in l + 1..=n {
            if c[r] / c[l] <= k {
                chmax!(ans, r - l);
            }
        }
    }
    println!("{}", ans);
}
