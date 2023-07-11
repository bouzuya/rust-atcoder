use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort_by_key(|&a_i| -a_i);
    let ans = a
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, a_i)| a_i)
        .sum::<i64>();
    println!("{}", ans);
}
