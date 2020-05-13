use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.iter().step_by(2).map(|&c| c).collect::<String>();
    println!("{}", ans);
}
