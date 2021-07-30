use proconio::input;

fn main() {
    input! {
        n: usize,
        vwxyz: [[i64; 5]; n],
    };
    let ans = vwxyz
        .into_iter()
        .filter(|i| i.iter().sum::<i64>() < 20)
        .count();
    println!("{}", ans);
}
