use proconio::input;

fn main() {
    input! {
        n: f64,
    };
    let mut ok = 100_000_f64;
    let mut ng = 0_f64;
    for _ in 0..100 {
        let x = ng + (ok - ng) / 2_f64;
        if x.powf(3_f64) + x >= n {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
