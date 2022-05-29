use std::collections::HashMap;

use proconio::input;

fn f(memo: &mut HashMap<usize, usize>, x: usize) -> usize {
    if let Some(&v) = memo.get(&x) {
        return v;
    }
    let a = x / 2;
    let b = x - a;
    let v = if a == 0 || a == 1 {
        x
    } else {
        f(memo, a) * f(memo, b) % 998_244_353
    };
    memo.insert(x, v);
    v
}

fn main() {
    input! {
        x: usize,
    };
    let mut memo = HashMap::new();
    let ans = f(&mut memo, x);
    println!("{}", ans);
}
