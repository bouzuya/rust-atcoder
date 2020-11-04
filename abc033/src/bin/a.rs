use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    };
    let set = n.into_iter().collect::<std::collections::HashSet<char>>();
    let ans = if set.len() == 1 { "SAME" } else { "DIFFERENT" };
    println!("{}", ans);
}
