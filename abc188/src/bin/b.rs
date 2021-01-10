use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    };
    let mut x = 0_i64;
    for (&a_i, &b_i) in a.iter().zip(b.iter()) {
        x += a_i * b_i;
    }
    let ans = x == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
