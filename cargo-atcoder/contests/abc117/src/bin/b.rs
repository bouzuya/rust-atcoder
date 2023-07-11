use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [u64; n],
    };
    let sum = l.iter().sum::<u64>();
    let max = *l.iter().max().unwrap();
    let ans = if sum - max > max { "Yes" } else { "No" };
    println!("{}", ans);
}
