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
        .iter()
        .copied()
        .zip(b.iter().copied())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>();
    println!("{}", ans);
}
