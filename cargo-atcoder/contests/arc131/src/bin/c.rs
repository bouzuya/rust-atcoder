use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let x = a.iter().copied().fold(0, |acc, x| acc ^ x);
    let ans = n % 2 != 0 || a.into_iter().any(|a_i| (a_i ^ x) == 0);
    println!("{}", if ans { "Win" } else { "Lose" });
}
