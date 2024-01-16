use std::collections::{HashMap, HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        capital_x: i64,
        capital_y: i64,
        xy: [(i64, i64); n],
    };

    let inf = 1_usize << 60;

    let ng = xy.into_iter().collect::<HashSet<(i64, i64)>>();
    let mut dist = HashMap::new();
    let mut deque = VecDeque::new();
    deque.push_back(((0_i64, 0_i64), 0_usize));
    dist.insert((0, 0), 0_usize);
    while let Some(((x, y), d)) = deque.pop_front() {
        if d > *dist.get(&(x, y)).unwrap() {
            continue;
        }
        let dir = vec![(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)];
        for (dx, dy) in dir {
            let (nx, ny, nd) = (x + dx, y + dy, d + 1);
            if !(-201_i64..=201_i64).contains(&nx) || !(-201_i64..=201_i64).contains(&ny) {
                continue;
            }
            if ng.contains(&(nx, ny)) {
                continue;
            }
            let min = *dist.get(&(nx, ny)).unwrap_or(&inf);
            if nd < min {
                dist.insert((nx, ny), nd);
                deque.push_back(((nx, ny), nd));
            }
        }
    }

    let ans = dist
        .get(&(capital_x, capital_y))
        .map(|x| *x as i64)
        .unwrap_or(-1_i64);
    println!("{}", ans);
}
