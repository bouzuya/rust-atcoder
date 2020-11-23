use proconio::input;

fn main() {
    input! {
        s_x: i64,
        s_y: i64,
        g_x: i64,
        g_y: i64,
    };
    let a = (g_y + s_y) as f64 / (g_x - s_x) as f64;
    let b = g_y as f64 - a * g_x as f64;
    let ans = -b / a;
    println!("{:.10}", ans);
}
