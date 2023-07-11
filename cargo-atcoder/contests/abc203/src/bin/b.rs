use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut sum = 0_usize;
    for i in 1..=n {
        for j in 1..=k {
            sum += i * 100 + j;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
