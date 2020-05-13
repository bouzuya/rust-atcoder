use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut map = std::collections::BTreeMap::new();
    for &a_i in a.iter() {
        *map.entry(a_i - 1).or_insert(0) += 1;
        *map.entry(a_i).or_insert(0) += 1;
        *map.entry(a_i + 1).or_insert(0) += 1;
    }
    let ans = map.values().max().unwrap();
    println!("{}", ans);
}
