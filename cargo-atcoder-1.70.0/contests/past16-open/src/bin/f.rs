use ac_library::ModInt998244353 as ModInt;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut prev = ModInt::new(1);
    let mut curr = ModInt::new(0);
    for c in s {
        if c == '*' {
            prev *= curr;
            curr = ModInt::new(0);
        } else {
            curr *= 10;
            curr += (c as u8 - b'0') as u64;
        }
    }
    prev *= curr;

    let ans = prev;
    println!("{}", ans);
}
