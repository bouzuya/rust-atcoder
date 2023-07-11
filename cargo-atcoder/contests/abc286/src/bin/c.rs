use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut s: Chars,
    };

    let f = |s: &[char]| -> usize {
        let mut sum = 0_usize;
        for i in 0..n / 2 {
            if s[i] != s[n - 1 - i] {
                sum += b;
            }
        }
        sum
    };

    let mut min = f(&s);
    for i in 1..=n {
        s.rotate_left(1);
        min = min.min(i * a + f(&s));
    }
    let ans = min;
    println!("{}", ans);
}
