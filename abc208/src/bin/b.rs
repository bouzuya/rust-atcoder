use proconio::input;

fn main() {
    input! {
        mut p: usize,
    };
    let coins = (1..=10)
        .map(|i| (1..=i).product::<usize>())
        .collect::<Vec<usize>>();
    let mut count = 0_usize;
    for coin in coins.into_iter().rev() {
        count += p / coin;
        p %= coin;
    }
    let ans = count;
    println!("{}", ans);
}
