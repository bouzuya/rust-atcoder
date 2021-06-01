use proconio::input;

fn main() {
    input! {
        p: f64,
    };
    let f = |x: f64| x + p / f64::powf(2_f64, x / 1.5_f64);
    let mut l = 0_f64;
    let mut r = p;
    for _ in 0..200 {
        let p1 = l + (r - l) / 3_f64;
        let p2 = r - (r - l) / 3_f64;
        if f(p1) < f(p2) {
            r = p2;
        } else {
            l = p1;
        }
    }
    let ans = f(l);
    println!("{}", ans);
}
