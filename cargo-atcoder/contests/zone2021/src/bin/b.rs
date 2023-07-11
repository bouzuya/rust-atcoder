use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        h: f64,
        dh: [(f64, f64); n],
    };
    let mut max = 0_f64;
    for &(d_i, h_i) in dh.iter() {
        let x = h_i - d_i * (h - h_i) / (d - d_i);
        if x > max {
            max = if x < 0_f64 { 0_f64 } else { x };
        }
    }
    let ans = max;
    println!("{}", ans)
}
