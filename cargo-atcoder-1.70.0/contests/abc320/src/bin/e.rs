use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(usize, usize, usize); m],
    };
    // 0: 流す
    // 1: 戻る
    let mut events = tws
        .into_iter()
        .map(|(t, w, s)| (Reverse(t), 0, w, s))
        .collect::<BinaryHeap<_>>();
    let mut index = (0..n).map(Reverse).collect::<BinaryHeap<Reverse<usize>>>();

    let mut ans = vec![0_usize; n];
    while let Some((Reverse(t), ty, w, s)) = events.pop() {
        match ty {
            0 => {
                if let Some(Reverse(i)) = index.pop() {
                    ans[i] += w;
                    events.push((Reverse(t + s), 1, i, 0));
                }
            }
            1 => {
                let i = w;
                index.push(Reverse(w));
            }
            _ => unreachable!(),
        }
    }

    for a in ans {
        println!("{a}");
    }
}
