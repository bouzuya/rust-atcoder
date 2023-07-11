use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let pos = s.iter().skip(1).step_by(4).position(|&c| c == 'o');
    let ans = pos
        .map(|c| format!("{}", c + 1))
        .unwrap_or_else(|| "none".to_string());
    println!("{}", ans);
}
