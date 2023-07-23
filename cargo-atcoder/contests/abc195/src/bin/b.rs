use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    };
    let w = w * 1000;
    let c = w / a;
    if c * b < w {
        println!("UNSATISFIABLE");
        return;
    }
    let max = c;
    let min = (w + b - 1) / b;
    println!("{} {}", min, max);
}
