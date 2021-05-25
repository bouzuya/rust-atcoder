use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let ans = s.iter().filter(|&&c| c == 'R').count();
    let ans = if ans != 2 {
        ans
    } else {
        if s[1] == 'R' {
            2
        } else {
            1
        }
    };
    println!("{}", ans);
}
