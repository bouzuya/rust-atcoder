use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let ans = a.iter().map(|a_i| a_i - 1).sum::<i64>();
    println!("{}", ans);
}
