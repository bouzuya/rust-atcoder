use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    let n = s.len();
    let mut max = 0_usize;
    let mut count = 0_usize;
    let mut r = 0_usize;
    for l in 0..n {
        while (r < n) && ((s[r] == 'X') || (s[r] == '.' && count < k)) {
            count += if s[r] == '.' { 1 } else { 0 };
            r += 1;
        }
        max = max.max(r - l);
        if r == l {
            r += 1;
        } else {
            if s[l] == '.' {
                count -= 1;
            }
        }
    }
    let ans = max;
    println!("{}", ans);
}
