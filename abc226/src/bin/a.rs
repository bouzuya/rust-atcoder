use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };
    let x = x
        .into_iter()
        .filter(|&c| c != '.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let ans = (x + 500) / 1000;
    println!("{}", ans);
}
