use proconio::input;

fn f(a: &[i64]) -> i64 {
    let n = a.len() as i64;
    let mut b = a.iter().copied().collect::<Vec<i64>>();
    b.sort();
    b.iter()
        .copied()
        .enumerate()
        .map(|(i, b_i)| i as i64 * b_i - b_i * (n - 1 - i as i64))
        .sum::<i64>()
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let sum_x = f(&xy.iter().copied().map(|(x, _)| x).collect::<Vec<i64>>());
    let sum_y = f(&xy.iter().copied().map(|(_, y)| y).collect::<Vec<i64>>());
    let ans = sum_x + sum_y;
    println!("{}", ans);
}
