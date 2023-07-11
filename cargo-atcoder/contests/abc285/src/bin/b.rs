use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    for i in 1..n {
        let mut l = n - i;
        for k in 0..n - i {
            if s[k] == s[k + i] {
                l = k;
                break;
            }
        }
        println!("{}", l);
    }
}
