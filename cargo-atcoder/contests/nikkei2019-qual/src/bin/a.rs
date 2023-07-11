use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let max = std::cmp::min(a, b);
    let min = (b + a).saturating_sub(n);
    println!("{} {}", max, min);
}
