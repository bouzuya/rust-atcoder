use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    };

    let mut query = vec![];
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                query.push((t, x, y));
            }
            2 => {
                input! {
                    x: usize,
                }
                query.push((t, x, 0));
            }
            _ => unreachable!(),
        }
    }

    let mut next = HashMap::new();
    let mut prev = HashMap::new();
    for i in 0..n - 1 {
        let a_i = a[i];
        let a_j = a[i + 1];
        next.entry(a_i).or_insert(a_j);
        prev.entry(a_j).or_insert(a_i);
    }

    let head = 0_usize;
    let tail = 1 << 60_usize;
    next.insert(head, a[0]);
    prev.insert(a[0], head);
    next.insert(a[n - 1], tail);
    prev.insert(tail, a[n - 1]);

    for (t, x, y) in query {
        match t {
            1 => {
                // x -> next_x
                // => x -> y -> next_x
                let next_x = *next.get(&x).unwrap();
                next.insert(x, y);
                next.insert(y, next_x);
                prev.insert(y, x);
                prev.insert(next_x, y);
            }
            2 => {
                // prev_x -> x -> next_x
                // => prev_x -> next_x
                let prev_x = *prev.get(&x).unwrap();
                let next_x = *next.get(&x).unwrap();
                next.insert(prev_x, next_x);
                prev.insert(next_x, prev_x);
                next.remove(&x);
                prev.remove(&x);
            }
            _ => unreachable!(),
        }
    }

    let mut cur = next[&head];
    while cur != tail {
        println!("{}", cur);
        cur = next[&cur];
    }
}
