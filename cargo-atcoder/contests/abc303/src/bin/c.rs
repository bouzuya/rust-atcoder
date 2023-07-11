use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: i64,
        k: i64,
        s: Chars,
        xy: [(i64, i64); m],
    };

    let mut set = xy.into_iter().collect::<HashSet<(i64, i64)>>();
    let mut pos = (0, 0);
    for (i, s_i) in s.iter().copied().enumerate() {
        let (x, y) = pos;

        pos = match s_i {
            'R' => (x + 1, y),
            'L' => (x - 1, y),
            'U' => (x, y + 1),
            'D' => (x, y - 1),
            _ => unreachable!(),
        };
        h -= 1;
        if h < 0 {
            println!("No");
            return;
        }
        if h < k && set.remove(&pos) {
            h = k;
        }
    }

    println!("Yes");
}
