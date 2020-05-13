use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(u64, u64); n],
    };
    let ans = lr.iter().map(|&(l, r)| r - l + 1).sum::<u64>();
    println!("{}", ans);
}
