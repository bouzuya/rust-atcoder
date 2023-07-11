use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let ans = (0..n).fold(1, |acc, _| std::cmp::min(acc * 2, acc + k));
    println!("{}", ans);
}
