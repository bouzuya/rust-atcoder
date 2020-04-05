use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        av: [usize; n]
    };
    let sum = av.iter().sum::<usize>();
    let count = av.iter().filter(|&a| a * 4 * m >= sum).count();
    let ans = if count >= m { "Yes" } else { "No" };
    println!("{}", ans);
}
