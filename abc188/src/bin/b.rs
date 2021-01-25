use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    };
    let ans = a
        .iter()
        .zip(b.iter())
        .map(|(&a_i, &b_i)| a_i * b_i)
        .sum::<i64>()
        == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
