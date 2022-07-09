use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    };

    let d = d.to_radians();
    let x = a * d.cos() - b * d.sin();
    let y = a * d.sin() + b * d.cos();
    println!("{} {}", x, y);
}
