use std::collections::HashSet;

use num::rational::Ratio;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    };

    let mut set_w = HashSet::new();
    for x in 0.. {
        let g_a = 100 * a * x;
        if g_a > f {
            break;
        }
        for y in 0.. {
            let g_b = 100 * b * y;
            let g_w = g_a + g_b;
            if g_w > f {
                break;
            }
            set_w.insert(g_w);
        }
    }

    let mut set_s = HashSet::new();
    for x in 0.. {
        let g_c = c * x;
        if g_c > f {
            break;
        }
        for y in 0.. {
            let g_d = d * y;
            let g_s = g_c + g_d;
            if g_s > f {
                break;
            }
            set_s.insert(g_s);
        }
    }

    let mut max = Ratio::new(0, 1);
    let mut ans = (0, 0);
    for a in set_w {
        if a == 0 {
            continue;
        }
        for b in set_s.iter().copied() {
            if b * 100 > a * e {
                continue;
            }
            if a + b > f {
                continue;
            }
            let r = Ratio::new(b, a + b);
            if r >= max {
                max = r;
                ans = (a + b, b);
            }
        }
    }

    println!("{} {}", ans.0, ans.1);
}
