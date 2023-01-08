use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut sum = 0_usize;
    for i in 1..=n {
        let m = n / i;
        sum += i * (1 + m) * m / 2;
    }
    let ans = sum;
    println!("{}", ans);
}
