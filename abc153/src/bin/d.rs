use std::collections::HashMap;

use proconio::input;

fn f(memo: &mut HashMap<usize, usize>, x: usize) -> usize {
    if x == 1 {
        return 1;
    }
    if let Some(count) = memo.get(&x) {
        return *count;
    }

    let count = 1 + f(memo, x / 2) * 2;
    memo.insert(x, count);
    count
}

fn main() {
    input! {
        h: usize,
    };
    let mut memo = HashMap::new();
    let ans = f(&mut memo, h);
    println!("{}", ans);
}
