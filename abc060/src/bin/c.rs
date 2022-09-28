use proconio::input;

fn main() {
    input! {
        n: usize,
        capital_t: usize,
        t: [usize; n],
    };
    let mut sum = 0_usize;
    for i in 0..n - 1 {
        sum += (t[i + 1] - t[i]).min(capital_t);
    }
    sum += capital_t;
    let ans = sum;
    println!("{}", ans);
}
