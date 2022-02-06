use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut e = 0_f64;
    for i in 1..=n {
        e += n as f64 / i as f64;
    }
    let ans = e;
    println!("{}", ans);
}
