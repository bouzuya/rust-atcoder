use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        y: Chars,
        r: Chars,
    };

    let f = |chars: Vec<char>| -> i64 {
        let n = chars.len();
        match chars.iter().position(|c| c == &'.') {
            Some(p) => chars
                .into_iter()
                .filter(|&c| c != '.')
                .chain("0".repeat(4 - (n - 1 - p)).chars())
                .collect::<String>(),
            None => chars
                .into_iter()
                .chain("0".repeat(4).chars())
                .collect::<String>(),
        }
        .parse::<i64>()
        .unwrap()
    };

    let m = 10_000_i64;
    let cx = f(x) + 10_000_000_000;
    let cy = f(y) + 10_000_000_000;
    let r = f(r);

    let mut ans = 0_i64;
    for x in (cx - r + m - 1) / m..=(cx + r) / m {
        let dx = (cx - x * m).abs();
        let dy = {
            let mut ok = 0;
            let mut ng = r + 1;
            while ng - ok > 1 {
                let dy = ok + (ng - ok) / 2;
                if dx.pow(2) + dy.pow(2) <= r.pow(2) {
                    ok = dy;
                } else {
                    ng = dy;
                }
            }
            ok
        };
        let top = (cy + dy) / m;
        let bottom = (cy - dy + m - 1) / m;
        ans += top - bottom + 1;
    }

    println!("{}", ans);
}
