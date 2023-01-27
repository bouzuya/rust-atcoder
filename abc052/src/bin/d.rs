use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        x: [usize; n],
    };
    let mut sum = 0_usize;
    for i in 1..n {
        let d = x[i] - x[i - 1];
        sum += (d * a).min(b);
    }
    let ans = sum;
    println!("{}", ans);
}
