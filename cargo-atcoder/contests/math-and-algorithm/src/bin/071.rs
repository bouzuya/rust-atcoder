use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n],
    };
    let mut max = 0_f64;
    for (a1, b1, c1) in abc.iter().copied() {
        for (a2, b2, c2) in abc.iter().copied() {
            if a1 * b2 == b1 * a2 {
                continue;
            }
            let (a1, b1, c1) = (a1 as f64, b1 as f64, c1 as f64);
            let (a2, b2, c2) = (a2 as f64, b2 as f64, c2 as f64);
            let x = (c2 * b1 - c1 * b2) / (a2 * b1 - a1 * b2);
            let y = (c2 * a1 - c1 * a2) / (b2 * a1 - b1 * a2);
            if abc
                .iter()
                .copied()
                .all(|(a, b, c)| a as f64 * x + b as f64 * y <= c as f64)
            {
                max = max.max(x + y);
            }
        }
    }
    let ans = max;
    println!("{}", ans);
}
