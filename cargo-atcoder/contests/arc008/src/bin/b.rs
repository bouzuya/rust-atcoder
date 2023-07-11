use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        _: usize,
        name: Chars,
        kit: Chars,
    };
    let mut required_map = std::collections::BTreeMap::new();
    for &block in name.iter() {
        *required_map.entry(block).or_insert(0) += 1;
    }
    let mut unit_map = std::collections::BTreeMap::new();
    for &block in kit.iter() {
        *unit_map.entry(block).or_insert(0) += 1;
    }
    let mut max_count = 0;
    for (block, required_count) in required_map.iter() {
        match unit_map.get(&block) {
            None => {
                println!("-1");
                return;
            }
            Some(count) => {
                max_count = std::cmp::max(max_count, (required_count + count - 1) / count);
            }
        }
    }
    let ans = max_count;
    println!("{}", ans);
}
