use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        x: usize,
        y: usize,
    };
    let a = (w * h) as f64 / 2_f64;
    let b = w == 2 * x && h == 2 * y;
    println!("{} {}", a, if b { 1 } else { 0 });
}
