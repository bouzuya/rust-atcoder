use std::collections::BinaryHeap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    };

    let mut ans = 0_usize;
    for bits in 0_usize..1 << h {
        let rows = bits.count_ones() as usize;
        if rows > k {
            continue;
        }
        let columns = k - rows;

        let hs = (0..h)
            .map(|i| ((bits >> i) & 1) == 1)
            .collect::<Vec<bool>>();

        let mut count_black = rows * w;
        let mut pq = BinaryHeap::new();
        for i in 0..w {
            let mut b = 0_usize;
            for j in 0..h {
                if hs[j] {
                    continue;
                }
                if c[j][i] == '#' {
                    b += 1;
                }
            }
            count_black += b;
            pq.push(h - rows - b);
        }

        for _ in 0..columns {
            count_black += pq.pop().unwrap();
        }

        ans = ans.max(count_black);
    }
    println!("{}", ans);
}
