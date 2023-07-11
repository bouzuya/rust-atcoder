use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans =
        s[0].to_string().parse::<usize>().unwrap() * s[2].to_string().parse::<usize>().unwrap();
    println!("{}", ans);
}
