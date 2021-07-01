use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        xy: [(i64, i64); n],
    };
    let f = |cx: f64| {
        let mut cost = 0_f64;
        for &(x_i, y_i) in xy.iter() {
            cost += (cx - x_i as f64).powf(2_f64) + (c - y_i).pow(2) as f64;
        }
        cost
    };
    let mut l = -100_000_f64;
    let mut r = 100_000_f64;
    for _ in 0..100 {
        let cl = (l * 2_f64 + r) / 3_f64;
        let cr = (l + r * 2_f64) / 3_f64;
        if f(cl) < f(cr) {
            r = cr;
        } else {
            l = cl;
        }
    }
    let ans = match (f(l), f(r)) {
        (l, r) if l < r => l,
        (l, r) if l > r => r,
        (l, _) => l,
    };
    println!("{}", ans);
}
