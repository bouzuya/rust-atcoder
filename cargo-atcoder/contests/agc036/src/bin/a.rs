use proconio::input;

fn main() {
    input! {
        s: usize,
    };
    let v = 1_000_000_000;
    let x = (v - (s % v)) % v;
    let y = (s + x) / v;
    println!("0 0 {} 1 {} {}", v, x, y);
}
