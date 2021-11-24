use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        n: usize,
    };
    let x = n.saturating_sub(a);
    let y = (2 * n).saturating_sub(b);
    let z = (3 * n).saturating_sub(c);
    println!("{} {} {}", x, y, z);
}
