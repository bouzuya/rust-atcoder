use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        a: usize,
        n: usize,
    };
    let max_x = 10_usize.pow(n.to_string().len() as u32) - 1;
    let inf = max_x + 1;
    let mut dist = vec![inf; max_x + 1];
    let mut deque = BinaryHeap::new();
    deque.push(Reverse((0, 1)));
    dist[1] = 0;
    while let Some(Reverse((d, x))) = deque.pop() {
        if dist[x] != d {
            continue;
        }

        let nx = x * a;
        if nx <= max_x && d + 1 < dist[nx] {
            deque.push(Reverse((d + 1, nx)));
            dist[nx] = d + 1;
        }

        let mut xs = x.to_string().chars().collect::<Vec<char>>();
        for i in 0..xs.len() {
            let x = xs.iter().collect::<String>().parse::<usize>().unwrap();
            if x >= 10 && x % 10 != 0 {
                xs.rotate_right(1);
                let nx = xs.iter().collect::<String>().parse::<usize>().unwrap();
                if d + 1 + i < dist[nx] {
                    deque.push(Reverse((d + 1 + i, nx)));
                    dist[nx] = d + 1 + i;
                }
            }
        }
    }
    println!(
        "{}",
        if dist[n] != inf {
            dist[n] as i64
        } else {
            -1_i64
        }
    );
}
