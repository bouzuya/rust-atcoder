use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut y = s[0..4].iter().collect::<String>().parse::<usize>().unwrap();
    let mut m = s[5..7].iter().collect::<String>().parse::<usize>().unwrap();
    let mut d = s[8..10]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let leap = |y: usize| (y % 400 == 0) || ((y % 4 == 0) && (y % 100 != 0));
    let dom = |y: usize, m: usize| -> usize {
        let d = if leap(y) {
            vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        d[m - 1]
    };

    let next = |(y, m, d): (usize, usize, usize)| -> (usize, usize, usize) {
        if dom(y, m) != d {
            (y, m, d + 1)
        } else if m == 12 {
            (y + 1, 1, 1)
        } else {
            (y, m + 1, 1)
        }
    };
    let ok = |(y, m, d): (usize, usize, usize)| -> bool {
        format!("{}{:02}{:02}", y, m, d)
            .chars()
            .collect::<HashSet<_>>()
            .len()
            == 2
    };
    while !ok((y, m, d)) {
        let (ny, nm, nd) = next((y, m, d));
        y = ny;
        m = nm;
        d = nd;
    }
    println!("{}/{:02}/{:02}", y, m, d);
}
