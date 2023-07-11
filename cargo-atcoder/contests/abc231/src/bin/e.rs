use std::collections::HashMap;

use proconio::input;

fn f(memo: &mut HashMap<(usize, usize), usize>, a: &[usize], x: usize, n: usize) -> usize {
    if n == 1 {
        return x;
    }

    if let Some(&y) = memo.get(&(x, n)) {
        return y;
    }

    let mut ans = x;
    let a_i = a[n - 1];
    if x % a_i == 0 {
        ans = ans.min(x / a_i);
    }

    let b = x / a_i;
    let r = x - (b * a_i);
    let c = f(memo, a, r, n - 1);
    ans = ans.min(b + c);

    let b = (x + a_i - 1) / a_i;
    let r = b * a_i - x;
    let c = f(memo, a, r, n - 1);
    ans = ans.min(b + c);

    memo.insert((x, n), ans);

    ans
}

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };
    if n == 1 {
        println!("{}", x);
        return;
    }
    assert!(n >= 2);
    let mut memo = HashMap::new();
    let ans = f(&mut memo, &a, x, n);
    println!("{}", ans);
}
