use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
    };
    let ans = t
        .iter()
        .map(|&c| if c == '?' { 'D' } else { c })
        .collect::<String>();
    println!("{}", ans);
}
