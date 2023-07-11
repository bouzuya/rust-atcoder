use std::cmp;

use proconio::input;
use superslice::Ext;

fn lis_lens(a: &[i64]) -> Vec<usize> {
    let mut lis = vec![];
    let mut res = vec![];
    for &a_i in a.iter() {
        let j = lis.upper_bound(&(a_i - 1));
        match j.cmp(&lis.len()) {
            cmp::Ordering::Less => {
                lis[j] = a_i;
            }
            cmp::Ordering::Equal => {
                lis.push(a_i);
            }
            cmp::Ordering::Greater => unreachable!(),
        }
        res.push(j + 1);
    }
    res
}

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };

    let l1 = lis_lens(&a);
    a.reverse();
    let mut l2 = lis_lens(&a);
    l2.reverse();

    let mut max = 0;
    for k in 0..n {
        // a[0..k] で最長の部分列の前半の長さ b1
        let b1 = l1[k];
        // a[k..n] で最長の部分列の後半の長さ b2
        let b2 = l2[k];
        max = cmp::max(max, b1 + b2 - 1);
    }
    let ans = max;
    println!("{}", ans);
}
