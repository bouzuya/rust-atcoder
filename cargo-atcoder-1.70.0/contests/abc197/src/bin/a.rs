use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    s.rotate_left(1);
    let ans = s.into_iter().collect::<String>();
    println!("{}", ans);
}
