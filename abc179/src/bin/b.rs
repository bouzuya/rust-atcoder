use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(i64, i64); n]
    };
    let b = d
        .iter()
        .map(|(d_i1, d_i2)| d_i1 == d_i2)
        .collect::<Vec<bool>>();
    let ans = b.windows(3).map(|w| w.iter().all(|&w_i| w_i)).any(|w| w);
    println!("{}", if ans { "Yes" } else { "No" });
}
