use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut count = 0_usize;
    for a in 1..n {
        count += (n - 1) / a;
    }
    let ans = count;
    println!("{}", ans);
}
