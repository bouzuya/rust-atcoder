use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    };

    let mut t = 0_usize;
    let mut dist_t = 0_usize;
    while t <= x {
        if a <= x - t {
            dist_t += a * b;
            t += a + c;
        } else {
            dist_t += (x - t) * b;
            t += a;
        }
    }

    let mut t = 0_usize;
    let mut dist_a = 0_usize;
    while t <= x {
        if d <= x - t {
            dist_a += d * e;
            t += d + f;
        } else {
            dist_a += (x - t) * e;
            t += d;
        }
    }

    let ans = if dist_t == dist_a {
        "Draw"
    } else if dist_t > dist_a {
        "Takahashi"
    } else {
        "Aoki"
    };
    println!("{}", ans);
}
