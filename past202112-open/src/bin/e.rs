use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let is_left = |c: char| -> bool {
        let x = (c as u8 - b'0') as usize;
        x != 0 && x <= 5
    };
    let mut sum = 500_usize;
    let mut p = n[0];
    for c in n.into_iter().skip(1) {
        sum += if p == c {
            301
        } else if is_left(p) == is_left(c) {
            210
        } else {
            100
        };
        p = c;
    }
    let ans = sum;
    println!("{}", ans);
}
