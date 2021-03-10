use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let ans = a
        .iter()
        .enumerate()
        .filter(|&(i, a_i)| ((i + 1) % 2 != 0) && (a_i % 2 != 0))
        .count();
    println!("{}", ans);
}
