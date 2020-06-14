use proconio::input;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        x: i64,
        n: usize,
        p: [i64; n],
    };
    let mut min_a_i = x;
    let mut ans = 0;
    for i in 0..=101 {
        if p.contains(&i) {
            continue;
        }
        let a_i = (x - i as i64).abs();
        if chmin!(min_a_i, a_i) {
            ans = i;
        }
    }
    println!("{}", ans);
}
