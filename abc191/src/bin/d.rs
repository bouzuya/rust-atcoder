use num::integer::Roots;
use proconio::input;

fn main() {
    input! {
        x: String,
        y: String,
        r: String,
    };
    let f = |s: String| -> i128 {
        let chars = s.chars().collect::<Vec<char>>();
        match chars.iter().position(|c| c == &'.') {
            Some(p) => chars
                .into_iter()
                .filter(|&c| c != '.')
                .chain("0".repeat(4 - (s.len() - 1 - p)).chars())
                .collect::<String>(),
            None => chars
                .into_iter()
                .chain("0".repeat(4).chars())
                .collect::<String>(),
        }
        .parse::<i128>()
        .unwrap()
    };

    let p = 10_000_i128;
    let cx = f(x);
    let cy = f(y);
    let r = f(r);

    let c = |n: i128, d: i128| -> i128 { (n + if n > 0 { d - 1 } else { 0 }) / d };
    let f = |n: i128, d: i128| -> i128 { (n - if n > 0 { 0 } else { d - 1 }) / d };

    let left = c(cx - r, p);
    let right = f(cx + r, p);
    let mut ans = 0;
    for x in left..=right {
        let dx = cx - x * p;
        let dy = (r.pow(2) - dx.pow(2)).sqrt();
        let top = f(cy + dy, p);
        let bottom = c(cy - dy, p);
        let count = top - bottom + 1;
        ans += count;
    }

    println!("{}", ans);
}
