use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    let mut ans = 0_f64;
    for i in 1..n {
        ans += 1_f64 / i as f64;
    }
    ans *= n as f64;
    println!("{}", ans);
}
