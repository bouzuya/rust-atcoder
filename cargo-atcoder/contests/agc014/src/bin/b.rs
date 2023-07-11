use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        ab: [(i64, i64); m],
    };
    let mut map = std::collections::BTreeMap::new();
    for &(a_i, b_i) in ab.iter() {
        *map.entry(a_i).or_insert(0) += 1;
        *map.entry(b_i).or_insert(0) += 1;
    }
    let ans = map.values().all(|count| count % 2 == 0);
    println!("{}", if ans { "YES" } else { "NO" });
}
