use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let i_a = s.iter().position(|&c| c == 'A').unwrap();
    let i_z = s.iter().rposition(|&c| c == 'Z').unwrap();
    let ans = s[i_a..i_z + 1].iter().count();
    println!("{}", ans);
}
