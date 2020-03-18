use proconio::input;
use proconio::marker::Usize1;

fn d(a: usize, b: usize) -> usize {
    std::cmp::max(a, b) - std::cmp::min(a, b)
}

fn g(mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize) -> usize {
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
    }
    for s in (1..=30).rev() {
        let b = 3usize.pow(s) / 3;
        let b0 = (x1 / b, y1 / b);
        let b1 = (x2 / b, y2 / b);
        if b0.1 != b1.1 {
            return 0;
        }
        if b0.1 % 3 == 1 && d(b0.0, b1.0) > 1 {
            return std::cmp::min(
                std::cmp::min(y1, y2) - (b * b0.1 - 1),
                (b * (b0.1 + 1)) - std::cmp::max(y1, y2),
            );
        }
    }
    0
}

fn f(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    d(x1, x2) + d(y1, y2) + 2 * std::cmp::max(g(x1, y1, x2, y2), g(y1, x1, y2, x2))
}

fn main() {
    input! {
        q: usize,
        qv: [(Usize1, Usize1, Usize1, Usize1); q],
    };
    for (a, b, c, d) in qv {
        let ans = f(a, b, c, d);
        println!("{}", ans);
    }
}
