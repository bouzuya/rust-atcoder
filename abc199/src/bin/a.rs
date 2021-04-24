use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = a * a + b * b < c * c;
    println!("{}", if ans { "Yes" } else { "No" });
}
