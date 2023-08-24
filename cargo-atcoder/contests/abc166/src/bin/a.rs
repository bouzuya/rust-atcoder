use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = match s[1] {
        'B' => 'R',
        'R' => 'B',
        _ => unreachable!(),
    };
    println!("A{}C", ans);
}
