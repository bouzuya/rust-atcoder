use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 9],
    };

    let mut set = HashSet::new();
    for r in 0..9_i64 {
        for c in 0..9_i64 {
            for dr in 0..9_i64 {
                for dc in 0..9_i64 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let mut points = vec![
                        (r, c),
                        (r + dr, c + dc),
                        (r + dr - dc, c + dc + dr),
                        (r + dr - dc - dr, c + dc + dr - dc),
                    ];
                    if points.iter().all(|&(i, j)| {
                        (0..9).contains(&i)
                            && (0..9).contains(&j)
                            && s[i as usize][j as usize] == '#'
                    }) {
                        points.sort();
                        set.insert(points);
                    }
                }
            }
        }
    }
    let ans = set.len();
    println!("{}", ans);
}
