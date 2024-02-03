use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut p1 = None;
    let mut p2 = None;
    for i in 0..n {
        for j in 0..n {
            match s[i][j] {
                'P' => {
                    if p1.is_none() {
                        p1 = Some((i, j));
                    } else {
                        p2 = Some((i, j));
                    }
                }
                '.' | '#' => {}
                _ => unreachable!(),
            }
        }
    }
    let (p1, p2) = (p1.unwrap(), p2.unwrap());

    let mut count = 0_usize;
    let max = 6_000_000;
    let mut dist = HashMap::new();
    let mut deque = VecDeque::new();
    let (p1, p2) = (p1.min(p2), p1.max(p2));
    dist.insert((p1, p2), 0_usize);
    deque.push_back((p1, p2));
    while let Some((p1c, p2c)) = deque.pop_front() {
        // 嘘解法
        count += 1;
        if count > max {
            break;
        }
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let p1n = (
                (p1c.0 as i64 + dr).clamp(0, (n - 1) as i64) as usize,
                (p1c.1 as i64 + dc).clamp(0, (n - 1) as i64) as usize,
            );
            let p1n = if s[p1n.0][p1n.1] == '#' { p1c } else { p1n };
            let p2n = (
                (p2c.0 as i64 + dr).clamp(0, (n - 1) as i64) as usize,
                (p2c.1 as i64 + dc).clamp(0, (n - 1) as i64) as usize,
            );
            let p2n = if s[p2n.0][p2n.1] == '#' { p2c } else { p2n };
            let (p1n, p2n) = (p1n.min(p2n), p1n.max(p2n));
            let d = dist[&(p1c, p2c)] + 1;
            if p1n == p2n {
                println!("{}", d);
                return;
            }
            if dist.contains_key(&(p1n, p2n)) {
                continue;
            }
            dist.insert((p1n, p2n), d);
            deque.push_back((p1n, p2n));
        }
    }

    println!("-1");
}
