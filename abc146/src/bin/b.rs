use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let chars = (0..26).map(|i| (b'A' + i) as char).collect::<Vec<char>>();
    let ans = s
        .iter()
        .map(|s_i| {
            let j = chars.iter().position(|c| c == s_i).unwrap();
            chars[(j + n) % chars.len()]
        })
        .collect::<String>();
    println!("{}", ans);
}
