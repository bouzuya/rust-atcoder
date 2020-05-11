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
    let mut ans = (0_f64, 0, 0);
    for x_a in 0..=f {
        if a * x_a * 100 > f {
            continue;
        }
        for x_b in 0..=(f - a * x_a * 100) {
            if b * x_b * 100 > f {
                continue;
            }
            let w = (a * x_a + b * x_b) * 100;
            if w == 0 || w > f {
                continue;
            }
            for x_c in 0..=f - w {
                if w + c * x_c > f {
                    continue;
                }
                for x_d in 0..=f - w {
                    if w + d * x_d > f {
                        continue;
                    }
                    let s = c * x_c + d * x_d;
                    if w + s > f || s * 100 > w * e {
                        continue;
                    }
                    let x = s as f64 * 100_f64 / (s as f64 + w as f64);
                    ans = if ans.0 <= x { (x, s + w, s) } else { ans };
                }
            }
        }
    }
    println!("{} {}", ans.1, ans.2);
}
