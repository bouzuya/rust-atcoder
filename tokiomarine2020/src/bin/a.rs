use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.iter().take(3).collect::<String>();
    println!("{}", ans);
}
