use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let sorted = (0..n).collect::<Vec<usize>>();
    let count = sorted
        .into_iter()
        .zip(p.into_iter())
        .filter(|&(a, b)| a != b)
        .count();
    let ans = count == 0 || count == 2;
    println!("{}", if ans { "YES" } else { "NO" });
}
