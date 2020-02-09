use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
        a: usize,
        b: usize,
        u: String,
    };
    println!(
        "{} {}",
        a - (if s == u { 1 } else { 0 }),
        b - (if t == u { 1 } else { 0 })
    );
}
