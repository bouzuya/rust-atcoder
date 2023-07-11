use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    };
    let e = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a * 2 + b * 4)
        .sum::<i64>() as f64
        / 6_f64;
    let ans = e;
    println!("{}", ans);
}
