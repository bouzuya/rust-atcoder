use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    };
    a.sort();
    b.sort();
    let ans = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a_i, b_i)| (a_i - b_i).abs())
        .sum::<i64>();
    println!("{}", ans);
}
