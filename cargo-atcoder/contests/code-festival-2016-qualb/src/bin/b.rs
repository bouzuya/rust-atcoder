use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    };
    let mut count_a = 0;
    let mut count_b = 0;
    for i in 0..n {
        let b = match s[i] {
            'a' => {
                if count_a + count_b < a + b {
                    count_a += 1;
                    true
                } else {
                    false
                }
            }
            'b' => {
                if count_a + count_b < a + b && count_b < b {
                    count_b += 1;
                    true
                } else {
                    false
                }
            }
            'c' => false,
            _ => unreachable!(),
        };
        println!("{}", if b { "Yes" } else { "No" });
    }
}
