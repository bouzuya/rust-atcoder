use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    };
    let mut ng = 1.0000000000001_f64;
    let mut ok = 2_f64;
    for _ in 0..100 {
        let x = (ok + ng) / 2_f64;
        if a * x.powf(5_f64) + b * x >= -c {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
