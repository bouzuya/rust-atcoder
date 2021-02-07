use proconio::input;

fn c(x: i64) -> i64 {
    (x + if x > 0 { 9_999 } else { 0 }) / 10_000
}

fn f(x: i64) -> i64 {
    (x - if x > 0 { 0 } else { 9_999 }) / 10_000
}

fn main() {
    input! {
        x: f64,
        y: f64,
        r: f64,
    };
    let x = (x * 10_000_f64).round() as i64;
    let y = (y * 10_000_f64).round() as i64;
    let r = (r * 10_000_f64).round() as i64;

    let mut count = 0_i64;
    let x_l = c(x - r);
    let x_r = f(x + r);
    for x_i in x_l..=x_r {
        let p = {
            let mut ng = r + 1;
            let mut ok = 0;
            while ng - ok > 1 {
                let m = ok + (ng - ok) / 2;
                let a = x - x_i * 10_000;
                let b = m;
                if a * a + b * b <= r * r {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            ok
        };
        let y_b = c(y - p);
        let y_t = f(y + p);
        count += y_t - y_b + 1;
    }

    let ans = count;
    println!("{}", ans);
}
