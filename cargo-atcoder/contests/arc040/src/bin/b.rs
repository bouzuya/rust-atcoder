use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        r: usize,
        mut s: Chars
    };

    let mut t = 0;
    let mut i = 0;
    while let Some(i_r) = s.iter().rposition(|&s_i| s_i == '.') {
        if i + r > i_r {
            for j in i..std::cmp::min(i + r, n) {
                s[j] = 'o';
            }
        } else if s[i] == '.' {
            for j in i..std::cmp::min(i + r, n) {
                s[j] = 'o';
            }
        } else {
            i += 1;
        }
        t += 1;
    }

    let ans = t;
    println!("{}", ans);
}
