use ordered_float::OrderedFloat;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let sum = a.iter().sum::<i64>();
    let avg = sum as f64 / n as f64;
    let mut b = a
        .iter()
        .enumerate()
        .map(|(i, &a_i)| (OrderedFloat((avg - a_i as f64).abs()), i))
        .collect::<Vec<(OrderedFloat<f64>, usize)>>();
    b.sort();
    let ans = b[0].1;
    println!("{}", ans);
}
