use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        c: char,
        a: Chars,
    };
    // W: 0
    // B: 1
    // R: 2
    //
    // W + W = W = 0 + 0 = 0
    // B + B = R = 1 + 1 = 2
    // R + R = B = 2 + 2 = 1 (mod 3)
    // W + B = B = 0 + 1 = 1
    // W + R = R = 0 + 2 = 2
    // B + R = W = 1 + 2 = 0 (mod 3)
    let f = |ch: char| -> usize {
        match ch {
            'W' => 0,
            'B' => 1,
            'R' => 2,
            _ => unreachable!(),
        }
    };

    let mut sum = 0_usize;
    for a_i in a {
        sum += f(a_i);
        sum %= 3;
    }
    let ans = sum == f(c);
    println!("{}", if ans { "Yes" } else { "No" });
}
