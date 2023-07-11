use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();
    let mut count = 0_usize;
    let mut i = 0_usize;
    while i < n {
        if i == n - 1 {
            count += 1;
            i += 1;
        } else if s[i] == s[i + 1] {
            count += if i == n - 2 { 1 } else { 2 };
            i += 3;
        } else {
            count += 1;
            i += 1;
        }
    }

    let ans = count;
    println!("{}", ans);
}
