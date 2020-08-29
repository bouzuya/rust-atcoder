use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut min_count = s.len();
    for i in 0..s.len() {
        let mut is_ok = true;
        let mut count = 0;
        for j in 0..t.len() {
            if i + j >= s.len() {
                is_ok = false;
                break;
            }
            if s[i + j] != t[j] {
                count += 1;
            }
        }
        if is_ok {
            min_count = std::cmp::min(min_count, count);
        }
    }
    let ans = min_count;
    println!("{}", ans);
}
