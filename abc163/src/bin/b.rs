use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        av: [i64;m],
    };
    let sum = av.iter().sum();
    let ans = if n < sum { -1 } else { n - sum };
    println!("{}", ans);
}
