use proconio::input;

struct P {
    x: i64,
    y: i64,
}

fn distance(p0: &P, p1: &P, p2: &P) -> f64 {
    (((p2.y - p1.y) * p0.x - (p2.x - p1.x) * p0.y + p2.x * p1.y - p2.y * p1.x).abs() as f64)
        / (((p2.y - p1.y).pow(2) + (p2.x - p1.x).pow(2)) as f64).sqrt()
}

fn main() {
    input! {
        x: i64,
        y: i64,
        n: usize,
        xy: [(i64, i64); n],
    };
    let p0 = P { x, y };
    let mut min_d = distance(
        &p0,
        &P {
            x: xy[0].0,
            y: xy[0].1,
        },
        &P {
            x: xy[n - 1].0,
            y: xy[n - 1].1,
        },
    );
    for w in xy.windows(2) {
        let (p1_x, p1_y) = w[0];
        let (p2_x, p2_y) = w[1];
        let p1 = P { x: p1_x, y: p1_y };
        let p2 = P { x: p2_x, y: p2_y };
        let d = distance(&p0, &p1, &p2);
        if d < min_d {
            min_d = d;
        }
    }
    let ans = min_d;
    println!("{}", ans);
}
