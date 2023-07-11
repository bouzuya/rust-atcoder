use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn next_pos(r: usize, c: usize, cr: usize, cc: usize, d: usize) -> (usize, usize, usize) {
    match d {
        0 => {
            if cc == 0 {
                (cr, c - 1, d)
            } else {
                (cr, cc - 1, d)
            }
        }
        1 => {
            if cc == c - 1 {
                (cr, 0, d)
            } else {
                (cr, cc + 1, d)
            }
        }
        2 => {
            if cr == 0 {
                (r - 1, cc, d)
            } else {
                (cr - 1, cc, d)
            }
        }
        3 => {
            if cr == r - 1 {
                (0, cc, d)
            } else {
                (cr + 1, cc, d)
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    input! {
        r: usize,
        c: usize,
        s: [Chars; r],
    };
    // visited[r][c][d][m]
    // r: 0-19
    // c: 0-19
    // d: 0-4. 0: <, 1: >, 2: ^, 3: v
    // m: 0-15
    let mut visited = vec![vec![vec![vec![false; 16]; 4]; c]; r];
    let mut q = VecDeque::new();
    q.push_back(((0, 0, 1), 0));
    while let Some(((cr, cc, cd), cm)) = q.pop_front() {
        if visited[cr][cc][cd][cm] {
            continue;
        }
        visited[cr][cc][cd][cm] = true;
        q.push_back(match s[cr][cc] {
            '<' => (next_pos(r, c, cr, cc, 0), cm),
            '>' => (next_pos(r, c, cr, cc, 1), cm),
            '^' => (next_pos(r, c, cr, cc, 2), cm),
            'v' => (next_pos(r, c, cr, cc, 3), cm),
            '_' => (next_pos(r, c, cr, cc, if cm == 0 { 1 } else { 0 }), cm),
            '|' => (next_pos(r, c, cr, cc, if cm == 0 { 3 } else { 2 }), cm),
            '?' => {
                for d in 0..4 {
                    q.push_back((next_pos(r, c, cr, cc, d), cm));
                }
                continue;
            }
            '.' => (next_pos(r, c, cr, cc, cd), cm),
            '@' => {
                println!("YES");
                return;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (
                next_pos(r, c, cr, cc, cd),
                (s[cr][cc] as u8 - '0' as u8) as usize,
            ),
            '+' => (
                next_pos(r, c, cr, cc, cd),
                if cm != 15 { cm + 1 } else { 0 },
            ),
            '-' => (
                next_pos(r, c, cr, cc, cd),
                if cm != 0 { cm - 1 } else { 15 },
            ),
            _ => unreachable!(),
        });
    }
    println!("NO");
}
