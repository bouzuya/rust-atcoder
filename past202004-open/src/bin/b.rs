use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut map = std::collections::HashMap::new();
    for &s_i in s.iter() {
        *map.entry(s_i).or_insert(0_i64) += 1_i64;
    }
    let mut counts = map.into_iter().collect::<Vec<(char, i64)>>();
    counts.sort_by_key(|(_, v)| -v);
    let ans = counts[0].0;
    println!("{}", ans);
}
