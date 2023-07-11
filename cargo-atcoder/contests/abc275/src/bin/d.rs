use std::collections::HashMap;

use proconio::input;

fn f(memo: &mut HashMap<usize, usize>, k: usize) -> usize {
    if let Some(f_k) = memo.get(&k) {
        return *f_k;
    }

    if k == 0 {
        return 1;
    }

    let f_k = f(memo, k / 2) + f(memo, k / 3);
    memo.insert(k, f_k);
    f_k
}

fn main() {
    input! {
        n: usize,
    };
    let mut memo = HashMap::new();
    let ans = f(&mut memo, n);
    println!("{}", ans);
}
