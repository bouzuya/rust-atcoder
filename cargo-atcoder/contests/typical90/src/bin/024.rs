use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    };
    let mut d = 0;
    for (&a_i, &b_i) in a.iter().zip(b.iter()) {
        d += (a_i - b_i).abs();
    }

    let ans = d <= k && d % 2 == k % 2;
    println!("{}", if ans { "Yes" } else { "No" });
}
