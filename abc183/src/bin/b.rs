use proconio::input;

fn main() {
    input! {
        s: (f64, f64),
        g: (f64, f64),
    };
    let g = (g.0, -g.1);
    let dx = g.0 - s.0;
    let dy = g.1 - s.1;
    let a = dy / dx;
    let b = -a * g.0 + g.1;
    let x = -b / a;
    let ans = x;
    println!("{:.10}", ans);
}
