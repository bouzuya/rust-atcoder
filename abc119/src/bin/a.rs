use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let ans = if s <= "2019/04/30".chars().collect() {
        "Heisei"
    } else {
        "TBD"
    };
    println!("{}", ans);
}
