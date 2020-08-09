use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars
    };
    let modp = 1_000_000_007_i64;
    let mut c = vec![1_i64; 26];
    for &s_i in s.iter() {
        let c_i = (s_i as u8 - 'a' as u8) as usize;
        c[c_i] += 1;
    }
    let mut ans = 1_i64;
    for &c_i in c.iter() {
        ans *= c_i;
        ans %= modp;
    }
    ans -= 1;
    println!("{}", ans);
}
