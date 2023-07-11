use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    };
    let mut d = 0_i64;
    for (&a_i, &b_i) in a.iter().zip(b.iter()) {
        if a_i == b_i {
            continue;
        }
        if a_i < b_i {
            d += (b_i - a_i) / 2;
        } else {
            d += b_i - a_i;
        }
    }
    let ans = d >= 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
