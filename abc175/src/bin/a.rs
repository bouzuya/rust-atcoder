use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut count = 0;
    for s_i in s.iter().copied() {
        if s_i == 'R' {
            count += 1;
        }
    }
    let ans = if count == 2 {
        if s[1] == 'R' {
            2
        } else {
            1
        }
    } else {
        count
    };
    println!("{}", ans);
}
