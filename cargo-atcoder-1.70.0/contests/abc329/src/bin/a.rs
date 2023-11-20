use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s
        .into_iter()
        .map(|s_i| s_i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
