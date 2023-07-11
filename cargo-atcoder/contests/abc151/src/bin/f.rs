use proconio::input;

fn g(x: f64, y: f64, xy: &[(i64, i64)]) -> f64 {
    let mut max = 0_f64;
    for (x_i, y_i) in xy.iter().copied() {
        max = max.max(((x - x_i as f64).powf(2_f64) + (y - y_i as f64).powf(2_f64)).sqrt());
    }
    max
}

fn f(x: f64, xy: &[(i64, i64)]) -> f64 {
    let mut ly = 0_f64;
    let mut ry = 1_000_f64;
    for _ in 0..100 {
        let nly = (ly * 2_f64 + ry) / 3_f64;
        let nry = (ly + ry * 2_f64) / 3_f64;
        if g(x, nly, xy) > g(x, nry, xy) {
            ly = nly;
        } else {
            ry = nry;
        }
    }
    g(x, ry, xy)
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut lx = 0_f64;
    let mut rx = 1_000_f64;
    for _ in 0..100 {
        let nlx = (lx * 2_f64 + rx) / 3_f64;
        let nrx = (lx + rx * 2_f64) / 3_f64;
        if f(nlx, &xy) > f(nrx, &xy) {
            lx = nlx;
        } else {
            rx = nrx;
        }
    }
    let ans = f(rx, &xy);
    println!("{}", ans);
}
