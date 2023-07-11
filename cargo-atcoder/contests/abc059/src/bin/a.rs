use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 3]
    };
    let t = format!("{}{}{}", s[0][0], s[1][0], s[2][0]);
    println!("{}", t.to_uppercase());
}
