use std::cmp::max;

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        nml: [usize; 3],
        pqr: [usize; 3],
    };
    let mut max_count = 0;
    let mut a: Vec<usize> = nml.clone();
    a.sort();
    loop {
        let mut b: Vec<usize> = pqr.clone();
        b.sort();
        loop {
            let mut count = 1;
            for i in 0..3 {
                count *= a[i] / b[i];
            }
            max_count = max(max_count, count);
            if !b.next_permutation() {
                break;
            }
        }
        if !a.next_permutation() {
            break;
        }
    }
    let ans = max_count;
    println!("{}", ans);
}
