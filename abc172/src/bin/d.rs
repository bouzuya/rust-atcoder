use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = 0;
    for i in 1..=n {
        let m = n / i;
        ans += i * (m * (m + 1) / 2);
    }
    println!("{}", ans);
}
