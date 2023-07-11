use proconio::input;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q],
    };
    for e_i in e {
        let r = l / 2_f64;
        let x1 = 0_f64;
        let y1 = -r * ((e_i / t) * 2_f64 * std::f64::consts::PI).sin();
        let z1 = r - r * ((e_i / t) * 2_f64 * std::f64::consts::PI).cos();

        let d = ((x - x1) * (x - x1) + (y - y1) * (y - y1)).sqrt();
        let ans = z1.atan2(d) * 360_f64 / 2_f64 / std::f64::consts::PI;
        println!("{}", ans);
    }
}
