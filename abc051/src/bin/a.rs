use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s
        .into_iter()
        .map(|c| if c == ',' { ' ' } else { c })
        .collect::<String>();
    println!("{}", ans);
}
