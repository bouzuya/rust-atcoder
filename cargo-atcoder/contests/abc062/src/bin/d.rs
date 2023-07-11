use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 3 * n],
    };
    let inf = 1 << 60;
    let first = {
        let mut sum = 0_i64;
        let mut pq = BinaryHeap::new();
        for a_i in a.iter().copied().take(n) {
            pq.push(Reverse(a_i));
            sum += a_i;
        }
        let mut first = vec![-inf; n + 1];
        first[0] = sum;
        for i in 1..=n {
            let Reverse(x) = pq.pop().unwrap();
            sum -= x;
            let x = x.max(a[n - 1 + i]);
            sum += x;
            pq.push(Reverse(x));
            first[i] = sum;
        }
        first
    };

    let second = {
        let mut sum = 0_i64;
        let mut pq = BinaryHeap::new();
        for a_i in a.iter().copied().rev().take(n) {
            pq.push(a_i);
            sum += a_i;
        }
        let mut second = vec![inf; n + 1];
        second[0] = sum;
        for i in 1..=n {
            let x = pq.pop().unwrap();
            sum -= x;
            let x = x.min(a[2 * n - i]);
            sum += x;
            pq.push(x);
            second[i] = sum;
        }
        second.reverse();
        second
    };

    let mut ans = -inf;
    for i in 0..=n {
        ans = ans.max(first[i] - second[i]);
    }

    println!("{}", ans);
}
