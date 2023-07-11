use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let ans = n.into_iter().filter(|c| c == &'2').count();
    println!("{}", ans);
}
