use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut p = a[0];
    let mut sum = 0_i64;
    for &a_i in a.iter().skip(1) {
        sum += a_i - p;
        p = a_i;
    }
    let avg = sum as f64 / (n - 1) as f64;
    let ans = avg;
    println!("{}", ans);
}
