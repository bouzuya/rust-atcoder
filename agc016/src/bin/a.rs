use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut ans = 100;
    for i in 0..26 {
        let c = (i as u8 + 'a' as u8) as char;
        let (l, r) = {
            let mut max_l = 0;
            let mut l = 0;
            for &s_i in s.iter() {
                if s_i == c {
                    max_l = std::cmp::max(max_l, l);
                    l = 0;
                } else {
                    l += 1;
                }
            }
            (max_l, l)
        };
        let v = r + std::cmp::max(0, l - r);
        ans = std::cmp::min(ans, v);
    }
    println!("{}", ans);
}
