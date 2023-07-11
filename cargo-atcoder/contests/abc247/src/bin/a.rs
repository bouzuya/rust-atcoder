use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = format!("0{}", s[0..3].iter().collect::<String>());
    println!("{}", ans);
}
