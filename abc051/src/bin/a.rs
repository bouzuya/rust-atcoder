use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    println!(
        "{} {} {}",
        s[0..5].iter().collect::<String>(),
        s[6..13].iter().collect::<String>(),
        s[14..19].iter().collect::<String>()
    );
}
