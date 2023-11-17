use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let len = n.len();
    let ans = n
        .into_iter()
        .take(len.saturating_sub(2))
        .collect::<String>();
    println!("{}", if ans.is_empty() { "0".to_owned() } else { ans });
}
