use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    };
    let n = s.len();
    if n == 1 {
        println!("{}", 1);
        return;
    }
    let mut k = n;
    for (i, w) in s.windows(2).enumerate() {
        match w {
            [s_i, s_j] => {
                if s_i != s_j {
                    k = std::cmp::min(k, std::cmp::max(i + 1, n - (i + 1)));
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", k);
}
