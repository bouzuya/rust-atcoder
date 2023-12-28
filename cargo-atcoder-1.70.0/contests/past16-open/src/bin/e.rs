use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };
    let ans = x.len()
        - if x.starts_with(&['1']) && x.iter().skip(1).all(|c| c == &'0') {
            1
        } else {
            0
        };
    println!("{}", ans);
}
