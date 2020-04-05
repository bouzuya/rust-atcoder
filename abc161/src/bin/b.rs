use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        av: [usize; n]
    };
    let sum = av.iter().sum::<usize>();
    let x = av
        .iter()
        .fold(0, |acc, a| acc + if a * 4 * m >= sum { 1 } else { 0 });
    let ans = if x >= m { "Yes" } else { "No" };
    println!("{}", ans);
}
