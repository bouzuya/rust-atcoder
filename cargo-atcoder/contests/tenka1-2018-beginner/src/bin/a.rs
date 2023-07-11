use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = if s.len() == 2 {
        s.into_iter().collect::<String>()
    } else if s.len() == 3 {
        s.into_iter().rev().collect::<String>()
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
