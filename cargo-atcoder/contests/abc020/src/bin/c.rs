use proconio::input;
use proconio::marker::Bytes;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct C {
    r: usize,
    c: usize,
}

fn cost(x: usize, h: usize, w: usize, s: C, g: C, svv: &Vec<Vec<u8>>) -> usize {
    let enq =
        |pq: &mut BinaryHeap<Reverse<(usize, C)>>, uv: &Vec<Vec<bool>>, v: usize, n: C| -> () {
            if uv[n.r][n.c] {
                return;
            }
            let nv = match svv[n.r][n.c] {
                b'#' => x,
                b'.' | b'G' => 1,
                _ => unreachable!(),
            };
            pq.push(Reverse((v + nv, n)));
        };
    let mut uv = vec![vec![false; w]; h];
    uv[s.r][s.c] = true;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, s)));
    while let Some(Reverse((v, c))) = pq.pop() {
        if c == g {
            return v;
        }
        uv[c.r][c.c] = true;
        if c.r >= 1 {
            enq(&mut pq, &uv, v, C { r: c.r - 1, c: c.c });
        }
        if c.c + 1 < w {
            enq(&mut pq, &uv, v, C { r: c.r, c: c.c + 1 });
        }
        if c.r + 1 < h {
            enq(&mut pq, &uv, v, C { r: c.r + 1, c: c.c });
        }
        if c.c >= 1 {
            enq(&mut pq, &uv, v, C { r: c.r, c: c.c - 1 });
        }
    }
    unreachable!();
}

fn main() {
    input! {
        h: usize,
        w: usize,
        t: usize,
        svv: [Bytes; h],
    };
    let mut s = C { r: h, c: w };
    let mut g = C { r: h, c: w };
    for r in 0..h {
        for c in 0..w {
            match svv[r][c] {
                b'S' => s = C { r, c },
                b'G' => g = C { r, c },
                _ => {}
            }
        }
    }
    let mut ok = 1;
    let mut ng = t;
    while ng - ok > 1 {
        let x = ok + (ng - ok) / 2;
        if cost(x, h, w, s, g, &svv) <= t {
            ok = x;
        } else {
            ng = x;
        }
    }
    println!("{}", ok);
}
