use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let mut ans = 0;
    for (i, &s_i) in s.iter().enumerate() {
        let d = (s_i as usize - '0' as usize) as i64;
        if i % 2 == 0 {
            ans += d;
        } else {
            ans -= d;
        }
    }
    println!("{}", ans);
}
