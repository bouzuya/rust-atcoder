use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize,
        large_n: usize,
    };
    let mut sum = large_n;
    let mut c = large_n;
    while c >= m {
        let p = c / m * n;
        c = c % m + p;
        sum += p;
    }
    let ans = sum;
    println!("{}", ans);
}
