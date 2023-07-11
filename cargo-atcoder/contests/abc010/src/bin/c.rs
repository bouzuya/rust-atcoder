use proconio::input;

fn main() {
    input! {
        tx_a: i64,
        ty_a: i64,
        tx_b: i64,
        ty_b: i64,
        t: i64,
        v: i64,
        n: usize,
        xy: [(i64, i64); n],
    };
    for (x_i, y_i) in xy {
        let d1 = (x_i - tx_a).pow(2) + (y_i - ty_a).pow(2);
        let d2 = (tx_b - x_i).pow(2) + (ty_b - y_i).pow(2);
        let d = (d1 as f64).sqrt() + (d2 as f64).sqrt();
        if d <= (t * v) as f64 {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
