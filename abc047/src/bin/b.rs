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
        w: i64,
        h: i64,
        n: usize,
        xya: [(i64, i64, i64); n],
    };
    let mut l = 0_i64;
    let mut r = w;
    let mut b = 0_i64;
    let mut t = h;
    for (x_i, y_i, a_i) in xya {
        match a_i {
            1 => chmax!(l, x_i),
            2 => chmin!(r, x_i),
            3 => chmax!(b, y_i),
            4 => chmin!(t, y_i),
            _ => unreachable!(),
        };
    }
    let ans = if r - l > 0 && t - b > 0 {
        (r - l) * (t - b)
    } else {
        0
    };
    println!("{}", ans);
}
