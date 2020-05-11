use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n],
    };
    let ans = x
        .iter()
        .map(|&x_i| std::cmp::min(x_i, k - x_i) * 2)
        .sum::<usize>();
    println!("{}", ans);
}
