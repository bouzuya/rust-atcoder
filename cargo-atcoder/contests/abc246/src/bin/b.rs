use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let t = (b as f64).atan2(a as f64);
    let (x, y) = (t.cos(), t.sin());
    println!("{} {}", x, y);
}
