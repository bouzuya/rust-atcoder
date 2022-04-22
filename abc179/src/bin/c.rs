use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut sum = 0_usize;
    for a in 1..=n {
        sum += (n - 1) / a;
    }
    let ans = sum;
    println!("{}", ans);
}
