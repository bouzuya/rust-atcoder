use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut max = 0_usize;
    let n = s.len();
    for i in 0..n {
        for j in i..n {
            let len = j - i + 1;
            let mut ok = true;
            for k in 0..len / 2 {
                if s[i + k] != s[j - k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                max = max.max(len);
            }
        }
    }
    let ans = max;
    println!("{}", ans);
}
