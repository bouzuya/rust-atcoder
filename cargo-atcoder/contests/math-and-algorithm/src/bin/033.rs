use proconio::input;

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
    };

    let dist = |p1: (i64, i64), p2: (i64, i64)| -> f64 {
        (((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)) as f64).sqrt()
    };

    let ba = (a.0 - b.0, a.1 - b.1);
    let bc = (c.0 - b.0, c.1 - b.1);
    let ca = (a.0 - c.0, a.1 - c.1);
    let cb = (b.0 - c.0, b.1 - c.1);

    let babc = ba.0 * bc.0 + ba.1 * bc.1;
    let cacb = ca.0 * cb.0 + ca.1 * cb.1;
    let ans = if babc < 0 {
        dist(a, b)
    } else if cacb < 0 {
        dist(a, c)
    } else {
        let s = (ba.0 * bc.1 - ba.1 * bc.0).abs();
        s as f64 / dist(b, c)
    };

    println!("{}", ans);
}
