use proconio::input;

fn arithmetic_series(a_1: usize, d: usize, n: usize) -> usize {
    let a_n = a_1 + (n - 1) * d;
    (a_1 + a_n) * n / 2
}

fn main() {
    input! {
        n: usize,
    };
    let mut ans = 0_usize;
    for k in 1..=n {
        ans += arithmetic_series(k, k, n / k);
    }
    println!("{}", ans);
}
