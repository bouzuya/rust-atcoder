use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut map = std::collections::HashMap::new();
    for &c in s.iter() {
        *map.entry(c).or_insert(0) += 1;
    }
    let ans = map.len() == 2 && *map.iter().next().unwrap().1 == 2;
    println!("{}", if ans { "Yes" } else { "No" });
}
