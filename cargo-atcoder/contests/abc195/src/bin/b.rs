use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    };
    let w = w * 1000;
    if a > w {
        println!("UNSATISFIABLE");
        return;
    }
    let n = w / a;
    if n * a + n * (b - a) < w {
        println!("UNSATISFIABLE");
        return;
    }
    let max = n;
    let min = (w + b - 1) / b;
    println!("{} {}", min, max);
}
