use std::collections::BTreeMap;

use proconio::input;

fn f(memo: &mut BTreeMap<i64, usize>, x: i64, y: i64) -> usize {
    if y == 1 {
        return (x - y).abs() as usize;
    }
    if let Some(&r) = memo.get(&y) {
        return r;
    }
    let v = if y % 2 == 0 {
        f(memo, x, y / 2) + 1
    } else {
        f(memo, x, y + 1).min(f(memo, x, y - 1)) + 1
    }
    .min((x - y).abs() as usize);
    memo.entry(y).or_insert(v);
    v
}

fn main() {
    input! {
        x: i64,
        y: i64,
    };
    let mut memo = BTreeMap::new();
    let ans = f(&mut memo, x, y);
    println!("{}", ans);
}
