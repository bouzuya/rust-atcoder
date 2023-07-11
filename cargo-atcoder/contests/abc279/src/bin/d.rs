use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let f = |g: usize| -> f64 { (b * (g - 1)) as f64 + (a as f64) / (g as f64).sqrt() };
    let mut l = 1_usize;
    let mut r = 1 << 50;
    for _ in 0..1000 {
        let l2 = (l * 2 + r) / 3;
        let r2 = (l + r * 2) / 3;

        if b.checked_mul(r2 - 1).is_none() || f(l2) < f(r2) {
            r = r2;
        } else {
            l = l2;
        }
    }
    let ans = f(l).min(f(r)).min(f((r + l) / 2));
    println!("{}", ans);
}
