use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 3],
    };
    let ans = s
        .into_iter()
        .map(|s_i| s_i[0].to_ascii_uppercase())
        .collect::<String>();
    println!("{}", ans);
}
