use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        y: usize,
    };
    let mut sum = 0_usize;
    for i in 1..=n {
        if i <= k {
            sum += x;
        } else {
            sum += y;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
