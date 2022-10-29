use std::collections::HashMap;

use proconio::input;

fn f(memo: &mut HashMap<usize, usize>, x: usize) -> usize {
    if x == 0 {
        return 1;
    }
    if memo.contains_key(&x) {
        return *memo.get(&x).unwrap();
    }

    let v = f(memo, x / 2) + f(memo, x / 3);
    memo.insert(x, v);
    v
}

fn main() {
    input! {
        n: usize,
    };
    let ans = f(&mut HashMap::default(), n);
    println!("{}", ans);
}
