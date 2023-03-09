use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    };
    let ans = (1..=100_i64)
        .map(|p| x.iter().map(|x_i| (x_i - p).pow(2)).sum::<i64>())
        .min()
        .unwrap();
    println!("{}", ans);
}
