use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut count = 0;
    for i in 0..s.len() {
        if i + 4 <= s.len() && s[i..i + 4].iter().collect::<String>() == "ZONe" {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
