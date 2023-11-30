use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
    };

    // 対角線の長さが直径以下なら
    if h.pow(2) + w.pow(2) <= (2 * d).pow(2) {
        println!("1");
        return;
    }

    let f = |h: usize, d: usize| -> f64 {
        if h >= 2 * d {
            0_f64
        } else {
            let theta = (h as f64 / 2_f64 / d as f64).acos();
            let a1 = d.pow(2) as f64 * theta / 2_f64;
            let a2 = d.pow(2) as f64 * theta.sin() * theta.cos() / 2_f64;
            (a1 - a2) * 4_f64
        }
    };
    let area_circle = d.pow(2) as f64 * std::f64::consts::PI;
    let a1 = f(h, d);
    let a2 = f(w, d);
    let area_rectangle = (h * w) as f64;
    let ans = (area_circle - a1 - a2) / area_rectangle;
    println!("{}", ans);
}
